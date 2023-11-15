import { Direction } from "$lib/joystick";
import { minIndexByKey } from "$lib/util";

interface Widget {
    coords: {
        x: number;
        y: number;
        w: number;
        h: number;
    }
};

const TAU = 2 * Math.PI;

type Vec2 = [number, number];

// Vec2 center of a widget
const center = ({ coords }: Widget): Vec2 => [
    coords.x + coords.w / 2,
    coords.y + coords.h / 2
];

// u - v
const vec2diff = (u: Vec2, v: Vec2): Vec2 => [
    u[0] - v[0],
    u[1] - v[1]
];

// u == v
const vec2eq = (u: Vec2, v: Vec2): boolean => (
    u[0] === v[0] && u[1] === v[1]
);

// Center-to-center angle from widget a to b
const angle = (a: Widget, b: Widget) => {
    const ca = center(a);
    const cb = center(b);
    // Diff from a to b
    const d = vec2diff(cb, ca);

    return Math.atan2(d[1], d[0]);
}

// Absolute value of the difference between two angles
const absAngleDiff = (th1: number, th2: number) => (
    Math.min(
        Math.abs(th1 - th2),
        Math.abs(th1 - th2 + TAU)
    )
);

// Widget that is directionally "next" to another. This is determined by two
// parameters: the Manhattan distance from the center of the widget to another,
// and the angle from the center of the widget to another
export const widgetNeighborIndex = (widget: Widget, widgets: Widget[], direction: Direction): number | undefined => {
    // Direction
    // @ts-ignore
    const vecDir: Vec2 = {
        [Direction.Up]: [0, -1],
        [Direction.Down]: [0, 1],
        [Direction.Left]: [-1, 0],
        [Direction.Right]: [1, 0],
    }[direction];

    const angDir = Math.atan2(vecDir[1], vecDir[0]);

    const eligible = widgets.filter(widget_ => {
        if (vec2eq(center(widget), center(widget_))) return false;

        return absAngleDiff(angle(widget, widget_), angDir) <= TAU / 4 + 0.1;
    });

    const eligibleIndex = minIndexByKey(eligible, widget_ => {
        // Center-to-center distance and angle
        const vecC2C = vec2diff(center(widget_), center(widget));
        const manDist = Math.abs(vecC2C[0]) + Math.abs(vecC2C[1]);
        const angC2C = absAngleDiff(angle(widget, widget_), angDir);

        // This is just a hand-tuned metric to make navigation feel intuitive.
        // Works for most cases. Any weirdness can be explained by the fact that
        // I was sick when I wrote this.
        const key = manDist * (Math.pow((3 * angC2C / Math.PI), 3) + 1);
        return key;
    });

    // Retrieve the index in the original widget list, if it exists
    if (eligibleIndex === undefined) return undefined;
    const index = widgets.indexOf(eligible[eligibleIndex]);
    if (index === -1) return undefined;

    return index;
};