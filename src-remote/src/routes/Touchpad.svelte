<script lang="ts">
	import { connection } from "$lib/api";
	import { onMount } from "svelte";

    interface PT {
        startX: number;
        startY: number;
        startTime: number;
        lastX: number;
        lastY: number;
    }

    interface PTEvent {
        id: number;
        timeStamp: number;
        x: number;
        y: number;
    }

    const sens = 4.0;

    const pts = new Map<number, PT>();

    const onDown = (event: PTEvent) => {
        const { id, timeStamp, x, y } = event;

        pts.set(id, {
            startX: x,
            startY: y,
            startTime: timeStamp,
            lastX: x,
            lastY: y
        });
    };

    const onMove = (event: PTEvent) => {
        const { id, x, y } = event;
        const curr = pts.get(id)!;

        if (pts.size === 1) {
            const dx = sens * (x - curr.lastX);
            const dy = sens * (y - curr.lastY);
            $connection?.send({ MouseMove: { dx, dy } });
        }

        pts.set(id, {
            ...curr,
            lastX: x,
            lastY: y,
        });
    };

    const onUp = (event: PTEvent) => {
        // Check for single-taps
        const curr = pts.get(event.id)!;
        if (event.timeStamp - curr.startTime < 100) {
            if (pts.size === 1) {
                $connection?.send({ MousePress: 'LeftButton' });
            }
            else if (pts.size === 2) {
                $connection?.send({ MousePress: 'RightButton' });
                // Clear pts so that the next onUp call doesn't trigger another
                // press
                pts.clear();
            }
        }

        pts.delete(event.id);
    };

    const onCancel = (event: PTEvent) => {
        pts.delete(event.id);
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

    let p: any[] = [];
    const adaptTouch = (cb: (event: PTEvent) => void): ((event: TouchEvent) => void) => {
        return (event) => {
            p = [];
            for (const touch of event.changedTouches) {
                cb({
                    id: touch.identifier,
                    timeStamp: event.timeStamp,
                    x: touch.clientX,
                    y: touch.clientY
                });

                p.push(touch.identifier);
            }
        };
    }

    onMount(() => {
        const area = document.getElementById('toucharea')!;

        area.addEventListener('pointerdown', adaptPointer(onDown));
        area.addEventListener('pointermove', adaptPointer(onMove));
        area.addEventListener('pointerup', adaptPointer(onUp));
        area.addEventListener('pointercancel', adaptPointer(onCancel));

        area.addEventListener('touchstart', adaptTouch(onDown));
        area.addEventListener('touchmove', adaptTouch(onMove));
        area.addEventListener('touchend', adaptTouch(onUp));
        area.addEventListener('touchcancel', adaptTouch(onCancel));
    });
</script>

<div id="touchpad">
    <div id="toucharea" />
</div>
<p>{JSON.stringify(p)}</p>

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
        background-color: var(--bg2);
        width: 100%; /* Very landscape hostile */
        height: 100%;
        border-radius: 0.5rem;
    }
</style>