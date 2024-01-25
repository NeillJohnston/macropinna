//! Part of the state that defines capture for the visualizer.

use cpal::{Stream, SupportedStreamConfig, Device, SizedSample};
use std::{sync::{mpsc, Arc, Mutex, atomic::AtomicBool}, collections::VecDeque};
use tauri::State;

use crate::config_listener::ConfigManager;

// Human-audible range (https://en.wikipedia.org/wiki/Hearing_range)
const HEARING_RANGE: (f32, f32) = (20.0, 20_000.0);
// Just outside the hearing range (20Hz = 31.7mel, 20kHz = 3816.9mel)
const MEL_RANGE: (u32, u32) = (30, 3818);
const MEL_RESOLUTION: u32 = 4;
const DEFAULT_SMOOTHING: f32 = 0.5;
const DEFAULT_BUFFER_SIZE: usize = 8192;

pub struct AudioVisualizerManager {
    pub data: Arc<Mutex<Data>>,
    send: mpsc::Sender<AudioVisualizerMessage>,
}

struct AudioVisualizerHandler {
    recv: mpsc::Receiver<AudioVisualizerMessage>,
    stream: Option<Stream>,
    data: Arc<Mutex<Data>>,
    running: Arc<AtomicBool>
}

enum AudioVisualizerMessage {
    Pause,
    Unpause,
    SetDevice {
        name: Option<String>
    }
}

impl AudioVisualizerManager {
    pub fn new(config: &ConfigManager) -> anyhow::Result<Self> {
        let data = Data::new(
            MEL_RANGE,
            MEL_RESOLUTION,
            DEFAULT_SMOOTHING,
            DEFAULT_BUFFER_SIZE
        );
        let data = Arc::new(Mutex::new(data));

        let (send, recv) = mpsc::channel();
        let _data = data.clone();

        tokio::task::spawn(async {
            AudioVisualizerHandler {
                recv,
                stream: None,
                data: _data,
                running: Arc::new(AtomicBool::new(true))
            }.run();
        });

        let config = config.config.read().unwrap();
        let name = config.audio_device
            .as_ref()
            .map(|dev| dev.name.clone())
            .clone();

        send.send(AudioVisualizerMessage::SetDevice { name })?;

        Ok(AudioVisualizerManager { send, data })
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
    pub n_channels: usize,
    pub sample_rate: u32,
    pub buffer: VecDeque<f32>,
}

impl Data {
    fn new(range: (u32, u32), resolution: u32, smoothing: f32, buffer_size: usize) -> Self {
        Data {
            range,
            resolution,
            smoothing,
            n_channels: 0,
            sample_rate: 0,
            buffer: VecDeque::from(vec![0.0; buffer_size]),
        }
    }

    // Add new sample data to the buffer, s.t. we never go above the buffer's
    // original max capacity (a.k.a., no unnecessary allocations)
    fn update(&mut self, mut samples: Vec<f32>) {
        let len = samples.len();
        let capacity = self.buffer.capacity();

        if len > capacity {
            samples.drain(..(len - capacity));
        }

        self.buffer.drain(..samples.len());
        self.buffer.append(&mut samples.into());
    }

    fn process(&mut self) -> Vec<f32> {
        use spectrum_analyzer::{
            windows::hann_window,
            scaling::divide_by_N_sqrt,
            FrequencyLimit,
            samples_fft_to_spectrum,
        };

        let n_channels = self.n_channels;
        let sample_rate = self.sample_rate;

        // TODO best to use a specialized library for numerical processing
        // I think channel data comes interleaved - this just takes the
        // average across channels to produce a mono result
        let mut buf = Vec::with_capacity(self.buffer.len() / n_channels);
        for i in (0..self.buffer.len()).step_by(n_channels) {
            let mut s = 0.0;
            for j in i..(i + n_channels) {
                s += self.buffer[j];
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

        // let mut curr = self.range.0 + self.resolution;
        // let mut index = 0;
        // let mut accum = 0.0;
        // for (mel, val) in spectrum.to_mel_map().into_iter() {
        //     // "Max index" check might not be needed since ranges are just const now
        //     while mel > curr && index < self.data.len() - 1 {
        //         self.data[index] = accum;

        //         curr += self.resolution;
        //         index += 1;
        //         accum = 0.0;
        //     }

        //     accum += val;
        // }

        // self.data[index] = accum;

        let mut data = vec![0.0; ((self.range.1 - self.range.0) / self.resolution) as usize];
        for (mels, val) in spectrum.to_mel_map().into_iter() {
            let i = (mels / self.resolution) as usize;
            if let Some(accum) = data.get_mut(i) {
                *accum += val;
            }
        }

        data
    }
}

impl AudioVisualizerHandler {
    fn run(mut self) {
        loop {
            match self.recv.recv().unwrap() {
                AudioVisualizerMessage::Pause => {
                    self.running.store(false, std::sync::atomic::Ordering::Relaxed);
                },
                AudioVisualizerMessage::Unpause => {
                    self.running.store(true, std::sync::atomic::Ordering::Relaxed);
                },
                AudioVisualizerMessage::SetDevice { name } => {
                    self.load_device(&name).unwrap();
                }
            }
        }
    }

    fn load_device(&mut self, name: &Option<String>) -> anyhow::Result<()> {
        use cpal::{traits::*, SampleFormat};

        let host = cpal::default_host();
      
        let device = match name {
            Some(name) => host
                .devices()?
                .filter(|dev| match dev.name() {
                    Ok(_name) => &_name == name,
                    _ => false
                })
                .next()
                .ok_or(anyhow::anyhow!("Could not find audio device {}", name))?,
            None => host.default_output_device()
                .ok_or(anyhow::anyhow!("Could not find default output device"))?
        };

        let config = device.default_output_config()?;
        let stream = match config.sample_format() {
            SampleFormat::F32 => self.build_stream::<f32>(device, config),
            SampleFormat::I16 => self.build_stream::<i16>(device, config),
            // SampleFormat::I32 => self.build_stream::<i32>(device, config),
            _ => todo!()
        }?;
        stream.play()?;

        self.stream = Some(stream);

        Ok(())
    }

    fn build_stream<T: Into<f32> + SizedSample>(&self, device: Device, config: SupportedStreamConfig) -> anyhow::Result<Stream> {
        use cpal::traits::*;

        {
            let mut _data = self.data.lock().unwrap();
            _data.n_channels = config.channels() as usize;
            _data.sample_rate = config.sample_rate().0;
        }
        let data = self.data.clone();
        let running = self.running.clone();

        let stream = device.build_input_stream(
            &config.config(),
            move |samples: &[T], _info: &cpal::InputCallbackInfo| {
                if !running.load(std::sync::atomic::Ordering::Relaxed) { return; }

                let samples = samples
                    .into_iter()
                    .map(|x| <T as Into<f32>>::into(*x))
                    .collect();
                data.lock().unwrap().update(samples);
            },
            move |err| {
                log::error!("{}", err);
            },
            None
        )?;

        Ok(stream)
    }
}

#[derive(serde::Serialize)]
pub struct AudioSpectrumResponse {
    pub data: Vec<f32>
}

#[tauri::command]
pub fn get_audio_spectrum(audio_visualizer: State<'_, AudioVisualizerManager>) -> AudioSpectrumResponse {
    let data = audio_visualizer.data.lock().unwrap().process();
    AudioSpectrumResponse { data }
}

#[tauri::command]
pub fn pause_audio_visualizer(audio_visualizer: State<'_, AudioVisualizerManager>) {
    audio_visualizer.send.send(AudioVisualizerMessage::Pause).unwrap();
}

#[tauri::command]
pub fn unpause_audio_visualizer(audio_visualizer: State<'_, AudioVisualizerManager>) {
    audio_visualizer.send.send(AudioVisualizerMessage::Unpause).unwrap();
}