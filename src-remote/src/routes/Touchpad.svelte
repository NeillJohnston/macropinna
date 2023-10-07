<script lang="ts">
	import { connection } from "$lib/api";
	import { onMount } from "svelte";

    interface PT {
        startX: number;
        startY: number;
        startTime: number;
        lastTime: number;
        lastX: number;
        lastY: number;
        velX: number;
        velY: number;
    }

    interface PTEvent {
        id: number;
        timeStamp: number;
        x: number;
        y: number;
    }

    enum State {
        Idle,
        Moving,
        DragDelay,
        Dragging,
        Scrolling
    }

    let state = State.Idle;
    let timeout = setTimeout(() => {});

    const moveSens = 4.0;
    const scrollSens = 4.0;
    const velSmooth = 8.0;
    // const moveAccel = 1.05;
    // const scrollAccel = 1.0;
    const singleTapDelayMs = 100;
    const doubleTapDelayMs = 150;

    const pts = new Map<number, PT>();

    const onDown = (event: PTEvent) => {
        const { id, timeStamp, x, y } = event;

        if (state === State.Idle) {
            state = State.Moving;

            pts.set(id, {
                startX: x,
                startY: y,
                startTime: timeStamp,
                lastTime: timeStamp,
                lastX: x,
                lastY: y,
                velX: 0.0,
                velY: 0.0,
            });
        }
        else if (state === State.DragDelay) {
            clearTimeout(timeout);
            state = State.Dragging;

            pts.set(id, {
                startX: x,
                startY: y,
                startTime: timeStamp,
                lastTime: timeStamp,
                lastX: x,
                lastY: y,
                velX: 0.0,
                velY: 0.0,
            });
            $connection?.send({ MouseDown: 'LeftButton' });
        }
        else if (state === State.Moving) {
            state = State.Scrolling;

            pts.set(id, {
                startX: x,
                startY: y,
                startTime: timeStamp,
                lastTime: timeStamp,
                lastX: x,
                lastY: y,
                velX: 0.0,
                velY: 0.0,
            });
        }
    };

    const onMove = (event: PTEvent) => {
        const { id, x, y } = event;
        const curr = pts.get(id)!;

        const dxRaw = x - curr.lastX;
        const dyRaw = y - curr.lastY;

        const dt = event.timeStamp - curr.lastTime;
        const vs = 1/(1 + dt / velSmooth);
        const velX = vs * curr.velX + (1 - vs) * (dxRaw / dt);
        const velY = vs * curr.velY + (1 - vs) * (dyRaw / dt)

        pts.set(id, {
            ...curr,
            lastX: x,
            lastY: y,
            lastTime: event.timeStamp,
            velX,
            velY,
        });

        if (state === State.Moving || state === State.Dragging) {
            const dx = moveSens * velX * dt;
            const dy = moveSens * velY * dt;

            $connection?.send({ MouseMove: { dx, dy } });
        }
        else if (state === State.Scrolling) {
            const dx = 0;
            const dy = -scrollSens * velY * dt;

            $connection?.send({ MouseScroll: { dx, dy } });
        }
    };

    const onUp = (event: PTEvent) => {
        const curr = pts.get(event.id)!;

        if (state === State.Moving) {
            if (event.timeStamp - curr.startTime < singleTapDelayMs) {
                state = State.DragDelay;
                timeout = setTimeout(() => {
                    state = State.Idle;

                    $connection?.send({ MouseClick: 'LeftButton' });
                }, doubleTapDelayMs);
            }
            else {
                state = State.Idle;
            }
        }
        else if (state === State.Dragging) {
            state = State.Idle;
            
            $connection?.send({ MouseUp: 'LeftButton' });
        }
        else if (state === State.Scrolling) {
            state = State.Idle;
            if (event.timeStamp - curr.startTime < singleTapDelayMs) {
                $connection?.send({ MouseClick: 'RightButton' });
            }
        }

        pts.delete(event.id);
    };

    const onCancel = (event: PTEvent) => {
        // TODO might be a better way to handle this
        onUp(event);
    };

    const adaptPointer = (cb: (event: PTEvent) => void): ((event: PointerEvent) => void) => {
        return (event) => {
            cb({
                id: event.pointerId,
                timeStamp: event.timeStamp,
                x: event.clientX,
                y: event.clientY
            });
        };
    }

    const adaptTouch = (cb: (event: PTEvent) => void): ((event: TouchEvent) => void) => {
        return (event) => {
            for (const touch of event.changedTouches) {
                cb({
                    id: touch.identifier,
                    timeStamp: event.timeStamp,
                    x: touch.clientX,
                    y: touch.clientY
                });
            }
        };
    }

    onMount(() => {
        const area = document.getElementById('toucharea')!;

        area.addEventListener('pointerdown', adaptPointer(onDown));
        area.addEventListener('pointermove', adaptPointer(onMove));
        area.addEventListener('pointerup', adaptPointer(onUp));
        area.addEventListener('pointercancel', adaptPointer(onCancel));

        // TODO turning this on causes multiple events to be received for a
        // single touch (lol duh), need to switch based on browser capabilities
        // area.addEventListener('touchstart', adaptTouch(onDown));
        // area.addEventListener('touchmove', adaptTouch(onMove));
        // area.addEventListener('touchend', adaptTouch(onUp));
        // area.addEventListener('touchcancel', adaptTouch(onCancel));
    });
</script>

<div id="touchpad">
    <div id="toucharea" />
</div>

<style>
    #touchpad {
        position: relative;
        width: 100%; /* Very landscape hostile */
        height: auto;
        aspect-ratio: 1/1;
        box-sizing: border-box;
        border: 2rem solid transparent;
        touch-action: none;
    }

    #toucharea {
        background-color: var(--fg2);
        width: 100%; /* Very landscape hostile */
        height: 100%;
        border-radius: 0.5rem;
    }
</style>