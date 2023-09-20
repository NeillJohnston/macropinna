//! Part of the state that defines capture for the visualizer.

use cpal::{Stream, SupportedStreamConfig, Device, SizedSample};
use spectrum_analyzer::FrequencySpectrum;
use std::sync::{mpsc, Arc, Mutex};

// Human-audible range (https://en.wikipedia.org/wiki/Hearing_range)
const HEARING_RANGE: (f32, f32) = (20.0, 20_000.0);
// Just outside the hearing range (20Hz = 31.7mel, 20kHz = 3816.9mel)
const MEL_RANGE: (u32, u32) = (30, 3820);
const MEL_RESOLUTION: u32 = 1;

pub struct AudioVisualizerManager {
    pub data: Arc<Mutex<Data>>,
    _send: mpsc::Sender<AudioVisualizerMessage>,
}

struct AudioVisualizerHandler {
    recv: mpsc::Receiver<AudioVisualizerMessage>,
    stream: Option<Stream>,
    data: Arc<Mutex<Data>>
}

enum AudioVisualizerMessage {
    SetDevice {
        name: String,
        is_input: bool
    }
}

impl AudioVisualizerManager {
    pub fn new(device: &str) -> anyhow::Result<Self> {
        use std::thread::spawn;

        let data = Data::new(MEL_RANGE, MEL_RESOLUTION, 0.5);
        let data = Arc::new(Mutex::new(data));
        let (send, recv) = mpsc::channel();
        let _data = data.clone();

        spawn(move || {
            AudioVisualizerHandler {
                recv,
                stream: None,
                data: _data
            }.run();
        });

        let msg = AudioVisualizerMessage::SetDevice {
            name: device.to_string(),
            is_input: true
        };
        send.send(msg)?;

        Ok(AudioVisualizerManager { _send: send, data })
    }
}

/// Data that gets published for the visualizer.
/// 
/// Data gets formatted as a flattened list of evenly-spaced ranges of mels
/// (https://en.wikipedia.org/wiki/Mel_scale). Each range contains the sum of
/// amplitudes from the calculated spectrum.
/// 
/// In short, `data[i]` is the combination of audio data in the (mel-scaled)
/// frequency range `(range.0 + i*resolution, range.0 + (i+1)*resolution)`. Data
/// is also smoothed exponentially (without taking timing into consideration).
/// 
/// Given all that: the current constants (range = (30, 3820)) mean that each
/// of the labeled audio ranges take the following space in the dataset:
///   - lows: 7%
///   - low-mids: 13%
///   - midrange: 30%
///   - high-mids: 20%
///   - highs: 30%
/// That might be kind of small for the bass, UI can compensate.
pub struct Data {
    pub range: (u32, u32),
    pub resolution: u32,
    pub smoothing: f32,
    pub data: Vec<f32>,
}

impl Data {
    fn new(range: (u32, u32), resolution: u32, smoothing: f32) -> Self {
        let len = ((range.1 - range.0) / resolution) as usize;
        Data {
            range,
            resolution,
            smoothing,
            data: vec![0.0; len]
        }
    }

    // Incorporate new spectral data
    fn update(&mut self, spectrum: FrequencySpectrum) {
        let mut curr = self.range.0 + self.resolution;
        let mut index = 0;
        let mut accum = 0.0;
        for (mel, val) in spectrum.to_mel_map().into_iter() {
            // "Max index" check might not be needed since ranges are just const now
            while mel > curr && index < self.data.len() - 1 {
                self.data[index] =
                    accum * self.smoothing +
                    self.data[index] * (1.0 - self.smoothing);

                curr += self.resolution;
                index += 1;
                accum = 0.0;
            }

            accum += val;
        }

        self.data[index] =
            accum * self.smoothing +
            self.data[index] * (1.0 - self.smoothing);
    }
}

impl AudioVisualizerHandler {
    fn run(mut self) {
        loop {
            match self.recv.recv().unwrap() {
                AudioVisualizerMessage::SetDevice { name, is_input } => {
                    self.load_device(&name, is_input).unwrap();
                }
            }
        }
    }

    fn load_device(&mut self, name: &str, is_input: bool) -> anyhow::Result<()> {
        use cpal::{traits::*, SampleFormat};

        // TODO should host be configurable as well?
        let host = cpal::default_host();

        // let input_devices = host
        //     .devices()?
        //     .filter(|dev| dev.default_input_config().is_ok())
        //     .collect::<Vec<_>>();
        // let output_devices = host
        //     .devices()?
        //     .filter(|dev| dev.default_output_config().is_ok())
        //     .collect::<Vec<_>>();

        // for dev in input_devices.iter() {
        //     log::debug!("Available input device: {:?}", dev.name());
        // }
        // for dev in output_devices.iter() {
        //     log::debug!("Available output device: {:?}", dev.name());
        // }

        let device = host
            .devices()?
            .filter(|dev| match dev.name() {
                Ok(_name) => _name == name,
                _ => false
            })
            .next()
            .ok_or(anyhow::anyhow!("Could not find audio device {}", name))?;

        let config =
            if is_input {
                device.default_input_config()?
            }
            else {
                device.default_output_config()?
            };
        let stream = match config.sample_format() {
            SampleFormat::F32 => self.build_stream::<f32>(device, config, is_input),
            SampleFormat::I16 => self.build_stream::<i16>(device, config, is_input),
            // SampleFormat::I32 => self.build_stream::<i32>(device, config),
            _ => todo!()
        }?;
        stream.play()?;

        self.stream = Some(stream);

        Ok(())
    }

    fn build_stream<T: Into<f32> + SizedSample>(&self, device: Device, config: SupportedStreamConfig, is_input: bool) -> anyhow::Result<Stream> {
        use cpal::traits::*;

        let n_channels = config.channels() as usize;
        let data = self.data.clone();

        let stream =
            if is_input {
                device.build_input_stream(
                    &config.config(),
                    move |samples: &[T], _info: &cpal::InputCallbackInfo| {
                        Self::processor(
                            samples,
                            n_channels,
                            config.sample_rate().0,
                            data.clone()
                        );
                    },
                    move |err| {
                        log::error!("{}", err);
                    },
                    None
                )?
            }
            else {
                device.build_output_stream(
                    &config.config(),
                    move |samples: &mut[T], _info: &cpal::OutputCallbackInfo| {
                        Self::processor(
                            samples,
                            n_channels,
                            config.sample_rate().0,
                            data.clone()
                        );
                    },
                    move |err| {
                        log::error!("{}", err);
                    },
                    None
                )?
            };

        Ok(stream)
    }

    fn processor<T: Into<f32> + SizedSample>(
        samples: &[T],
        n_channels: usize,
        sample_rate: u32,
        data: Arc<Mutex<Data>>
    ) {
        use spectrum_analyzer::{
            windows::hann_window,
            scaling::divide_by_N_sqrt,
            FrequencyLimit,
            samples_fft_to_spectrum,
        };

        // TODO best to use a specialized library for numerical processing
        // I think channel data comes interleaved - this just takes the
        // average across channels to produce a mono result
        let mut buf = Vec::with_capacity(samples.len() / n_channels);
        for i in (0..samples.len()).step_by(n_channels) {
            let mut s = 0.0;
            for j in i..(i + n_channels) {
                s += samples[j].into();
            }
            buf.push(s);
        }
        for x in buf.iter_mut() {
            *x = *x / n_channels as f32;
        }

        let len = buf.len().next_power_of_two() / 2;

        // I assume this is fine (https://en.wikipedia.org/wiki/Window_function#Hann_and_Hamming_windows)
        let window = hann_window(&buf[..len]);
        let spectrum = samples_fft_to_spectrum(
            &window,
            sample_rate,
            FrequencyLimit::Range(HEARING_RANGE.0, HEARING_RANGE.1),
            // Commonly used function (https://docs.rs/spectrum-analyzer/1.4.0/spectrum_analyzer/scaling/index.html)
            Some(&divide_by_N_sqrt)
        ).unwrap();

        let mut data = data.lock().unwrap();
        data.update(spectrum);
    }
}