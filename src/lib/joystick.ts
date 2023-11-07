import { writable } from 'svelte/store';

export const nav = writable('/');

interface Target {
    // Alias this direction as another
    alias?: Direction;
    // Whether to keep the current stack top (combined with an id turns the
    // overall action into a stack push)
    keep?: boolean;
    // Target component id - can be just a string or a callback
    id?: string | (() => string | undefined);
    // Action to take after successfully navigating
    action?: () => void;
}

export interface Component {
    up?: Target;
    down?: Target;
    left?: Target;
    right?: Target;
    enter?: Target;
    exit?: Target;
    // An HTML id for scrolling - slashes will get automatically escaped.
    // Technically this can be any CSS selector
    scrollTo?: string;
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

    // Immediately set the top of the navigation stack
    setTop(id: string) {
        this.stack.pop();
        this.stack.push(id);
    }

    // Register a new navigable component
    register(id: string, component: Component) {
        this.components.set(id, component);
    }

    // Navigate in a given direction
    go(dir: Direction) {
        const shouldUpdate = this._go(dir);
        if (shouldUpdate) {
            const id = this.stack[this.stack.length - 1];
            nav.update(() => id);

            const scrollSelector = this.components.get(id)?.scrollTo;
            if (scrollSelector) {
                const escaped = scrollSelector.replaceAll('/', '\\/').replaceAll(':', '\\:');
                document.querySelector(escaped)?.scrollIntoView({
                    behavior: 'smooth',
                    block: 'center'
                });
            }
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
    focusEnter(focusId: string): Target {
        return {
            keep: true,
            id: focusId,
            action: () => {
                document.getElementById(focusId)?.focus();
            }
        }
    }

    // Exit target for a focusable input
    focusExit(focusId: string): Target {
        return {
            action: () => {
                document.getElementById(focusId)?.blur();
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
                const id = target.id();
                if (id) {
                    this.stack.push(id);
                }
            }
        }

        if (target.action) {
            target.action();
        }

        return true;
    }
}

export const joystick = new Joystick();

// Utility class to create and reference a list of elements that should navigate
// together like a list
export class NavList {
    private list: (string | undefined)[] = [];

    /*
    Elements is a list of elements that can be navigated through. Each
    element needs a unique name, and may optionally have a count if there are
    multiple instances of that element. For example:

        new NavList('myList', [
            { name: 'foo' },
            { name: 'bar', count: 3 },
            { name: 'baz' }
        ]);
    
    Will create a NavList with the following 5 elements:

        myList/foo
        myList/bar/0
        myList/bar/1
        myList/bar/2
        myList/baz
    */
    constructor(
        private prefix: string,
        elements: {
            name: string,
            count?: number
        }[]
    ) {
        this.list = [undefined];

        for (const { name, count } of elements) {
            if (count) {
                for (let index = 0; index < count; ++index) {
                    this.list.push(this.id(name, index));
                }
            }
            else {
                this.list.push(this.id(name));
            }
        }

        this.list.push(undefined);
    }

    private id(name: string, index?: number) {
        if (index === undefined) {
            return `${this.prefix}/${name}`;
        }
        else {
            return `${this.prefix}/${name}/${index}`;
        }
    }

    // Get the id, and prev/next (up/down) elements from the list
    get(name: string, index?: number): { id: string, up?: string, down?: string } {
        const id = this.id(name, index);
        const listIndex = this.list.indexOf(id);

        return {
            id,
            up: this.list[listIndex - 1],
            down: this.list[listIndex + 1]
        };
    }
}