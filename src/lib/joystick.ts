import { writable } from 'svelte/store';

export const nav = writable('/');

interface Target {
    // Alias this direction as another
    alias?: Direction;
    // Whether to keep the current stack top (combined with an id turns the
    // overall action into a stack push)
    keep?: boolean;
    // Target component id - can be just a string or a callback
    id?: string | (() => string);
    // Action to take after successfully navigating
    action?: () => void;
}

interface Component {
    up?: Target;
    down?: Target;
    left?: Target;
    right?: Target;
    enter?: Target;
    exit?: Target;
}

export enum Direction {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Exit
}

const targetKey = (dir: Direction): 'up' | 'down' | 'left' | 'right' | 'enter' | 'exit' => {
    return {
        [Direction.Up]: 'up',
        [Direction.Down]: 'down',
        [Direction.Left]: 'left',
        [Direction.Right]: 'right',
        [Direction.Enter]: 'enter',
        [Direction.Exit]: 'exit',
    }[dir] as 'up' | 'down' | 'left' | 'right' | 'enter' | 'exit';
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

    // Push onto the navigation stack, as if a component was just entered.
    push(id: string) {
        this.stack.push(id);
    }

    // Standard id for a focusable input
    focusId(id: string): string {
        return id + ':focus';
    }

    // Enter target for a focusable input
    focusEnter(id: string): Target {
        return {
            keep: true,
            id: this.focusId(id),
            action: () => {
                document.getElementById(this.focusId(id))?.focus();
            }
        }
    }

    // Exit target for a focusable input
    focusExit(id: string): Target {
        return {
            action: () => {
                document.getElementById(this.focusId(id))?.blur();
            }
        }
    }

    private _go(dir: Direction): boolean {
        const curr = this.stack.pop();
        if (!curr) throw new Error('Navigation stack is empty');

        const component = this.components.get(curr);
        if (!component) throw new Error(`Navigation component "${curr}" not found`);

        let target = component[targetKey(dir)];
        if (!target) {
            this.stack.push(curr);
            return false;
        }

        if (target.alias) {
            target = component[targetKey(target.alias)];
        }
        if (!target) {
            throw new Error(`Navigation target "${curr}}" (aliased) not found`);
        }
        if (target.alias) {
            throw new Error(`Multi-level aliasing detected for component "${curr}"`);
        }

        if (target.keep) {
            this.stack.push(curr);
        }

        if (target.id) {
            if (typeof target.id === 'string') {
                this.stack.push(target.id);
            }
            else {
                this.stack.push(target.id());
            }
        }

        if (target.action) {
            target.action();
        }

        return true;
    }
}

export const joystick = new Joystick();