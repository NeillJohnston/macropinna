import { writable } from 'svelte/store';

export const nav = writable('/');

interface Target {
    id: string;
    action?: () => void;
}

interface Component {
    up?: Target;
    down?: Target;
    left?: Target;
    right?: Target;
    enter?: Target;
    onExit?: () => void;
}

export enum Direction {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Exit
}

class Joystick {
    constructor(
        public stack: string[] = [],
        public components: Map<string, Component> = new Map()
    ) {}

    // Immediately set the navigation stack
    set(stack: string[]) {
        this.stack = stack;
    }

    // Register a new navigable component
    register(id: string, component: Component) {
        this.components.set(id, component);
    }

    // Navigate in a given direction
    go(dir: Direction) {
        const shouldUpdate = this._go(dir);
        if (shouldUpdate) {
            nav.update(() => this.stack[this.stack.length - 1]);
        }
    }

    // Navigate in a given direction from a specific spot on the stack
    goFrom(id: string, dir: Direction) {
        const index = this.stack.indexOf(id);
        if (index === -1) return;

        this.stack = this.stack.slice(0, index+1);
        this.go(dir);
    }

    // Convenience to create a callback that calls `goFrom`
    goFromCb(id: string, dir: Direction) {
        return () => this.goFrom(id, dir);
    }

    private _go(dir: Direction): boolean {
        const curr = this.stack.pop();
        if (!curr) throw new Error('Navigation stack is empty');

        if (dir === Direction.Exit) {
            if (this.stack.length === 0) {
                this.stack.push(curr);
                return false;
            }
            else {
                return true;
            }
        }

        const component = this.components.get(curr);
        if (!component) throw new Error(`Navigation component "${curr}" not found`);

        const dirKey = {
            [Direction.Up]: 'up',
            [Direction.Down]: 'down',
            [Direction.Left]: 'left',
            [Direction.Right]: 'right',
            [Direction.Enter]: 'enter',
        }[dir] as 'up' | 'down' | 'left' | 'right' | 'enter';

        const target = component[dirKey];
        if (!target) {
            this.stack.push(curr);
            return false;
        }
        if (dir === Direction.Enter) {
            this.stack.push(curr);
        }

        this.stack.push(target.id);
        if (target.action) {
            target.action();
        }

        return true;
    }
}

export const joystick = new Joystick();