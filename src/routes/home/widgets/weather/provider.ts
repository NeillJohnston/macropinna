import { invoke } from "@tauri-apps/api";
import strftime from "strftime";
import Qty from 'js-quantities';

export interface ProviderOptions {
    timeFormat: string;
    tempUnit: 'F' | 'C';
}

export const weatherItems = [
    {
        name: 'Current',
        key: 'current'
    },
    {
        name: '3 hour',
        key: 'forecast3hr'
    },
    {
        name: '6 hour',
        key: 'forecast6hr'
    },
    {
        name: 'Sunset',
        key: 'sunset'
    },
];

export const getWeather = async (options: ProviderOptions) => {
    const timeFmt = (t: number) => (
        strftime(options.timeFormat, new Date(1000 * t))
    );

    const tempFmt = (x: number, d?: number): string => {
        const to = {
            F: 'tempF',
            C: 'tempC'
        }[options.tempUnit];

        return Qty(x, 'tempK').to(to).scalar.toFixed(d ?? 0);
    };
    
    const res: any = await invoke('get_weather');
    if (!!res) {
        const extractForecast = (w: any) => ({
            time: timeFmt(w.dt),
            condition: fromOpenWeatherMapDesc(w.weather[0].id, w.weather[0].description),
            realTemp: tempFmt(w.main.temp),
            feelsLikeTemp: tempFmt(w.main.feels_like),
        });

        // TODO should switch by provider in the future, right now this
        // is hardcoded for OpenWeatherMap
        const cityName = res.current.name;

        const now = extractForecast(res.current);
        const later1 = extractForecast(res.forecast.list[0]);
        const later2 = extractForecast(res.forecast.list[1]);

        const sunsetTime = timeFmt(res.current.sys.sunset);

        return {
            fetched: now.time,
            current: `It's ${now.realTemp}°F (feels like ${now.feelsLikeTemp}°F) here in ${cityName}, with ${now.condition}.`,
            sunset: `Sunset today is at ${sunsetTime}.`,
            forecast3hr: `${later1.time}: expect ${later1.condition} and temperatures around ${later1.realTemp}°F.`,
            forecast6hr: `${later2.time}: expect ${later2.condition} and temperatures around ${later2.realTemp}°F.`
        };
    }
}

// Modifies the OpenWeatherMap descriptions so that they can be treated the same
// grammaticall (pluralizing any countable nouns)
const fromOpenWeatherMapDesc = (id: number, description: string) => {
    const _id = id.toString();
    if (/2../.test(_id)) {
        description = description.replace('thunderstorm', 'thunderstorms');
    }
    else if (/7../.test(_id)) {
        description = description.replace('tornado', 'tornadoes');
    }
    else if (/8../.test(_id)) {
        description = description.replace('sky', 'skies');
    }

    return description;
};