import { writable } from 'svelte/store';

type XAlign = 'left' | 'center' | 'right';
type YAlign = 'top' | 'middle' | 'bottom';

interface Layout {
    home: {
        clock?: Coords & Clock;
        weather?: Coords & Weather;
        todo?: Coords & Todo;
        audioVisualizer?: Coords & AudioVisualizer;
        player?: Coords & Player;
    }[];
}

interface Coords {
    coords: {
        x: number;
        y: number;
        w: number;
        h: number;
    }
}

export interface Clock {
    xAlign: XAlign;
    yAlign: YAlign;
}

export interface Weather {
    xAlign: XAlign;
    yAlign: YAlign;
}

export interface Todo {
    xAlign: XAlign;
}

export interface AudioVisualizer {
    yAlign: YAlign;
}

export interface Player {}

export const layout = writable({
    name: 'Neill',
    home: [
        {
            clock: {
                coords: { x: 0, y: 0, w: 6, h: 3 },
                xAlign: 'right',
                yAlign: 'middle'
            },
            weather: {
                coords: { x: 6, y: 0, w: 6, h: 3 },
                xAlign: 'left',
                yAlign: 'middle'
            },
            todo: {
                coords: { x: 6, y: 3, w: 6, h: 9 },
                xAlign: 'left',
            },
        },
        {
            audioVisualizer: {
                coords: { x: 2, y: 3, w: 8, h: 8 },
                yAlign: 'top'
            },
            player: {
                coords: { x: 2, y: 1, w: 8, h: 2 }
            }
        }
    ]
} as Layout);
