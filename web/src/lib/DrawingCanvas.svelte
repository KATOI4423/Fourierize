<script lang="ts">
    import { onMount } from "svelte";
    import init, { create_fourier_series_lettering } from "../../wasm";

    // WASM initialize
    onMount(async () => {
        await init();
        console.log("WASM initialized");
    });

    let canvas: HTMLCanvasElement;
    const SIZE = 1024;

    let ctx: CanvasRenderingContext2D;
    let drawing = false;

    function getPosition(event: PointerEvent) {
        const rect = canvas.getBoundingClientRect();

        return {
            x: (event.clientX - rect.left) * SIZE / rect.width,
            y: (event.clientY - rect.top) * SIZE / rect.height,
        };
    }
    function startDrawing(event: PointerEvent) {
        drawing = true;

        const { x, y } = getPosition(event);
        ctx.beginPath();
        ctx.moveTo(x, y);

        canvas.setPointerCapture(event.pointerId);
    }

    function draw(event: PointerEvent) {
        if (!drawing) {
            return;
        }

        const { x, y } = getPosition(event);
        ctx.lineTo(x, y);
        ctx.stroke();
    }

    function stopDrawing(event: PointerEvent) {
        if (!drawing) {
            return;
        }

        drawing = false;
        ctx.closePath();
        canvas.releasePointerCapture(event.pointerId);
    }

    function clearCanvas() {
        ctx.clearRect(0, 0, SIZE, SIZE);

        ctx.fillStyle = "white";
        ctx.fillRect(0, 0, SIZE, SIZE);
    }

    function getPixelData(): Uint8Array {
        const imageData = ctx.getImageData(0, 0, SIZE, SIZE);
        return new Uint8Array(imageData.data);
    }

    onMount(() => {
        ctx = canvas.getContext("2d", {
            willReadFrequently: true,
        })!;
        ctx.lineWidth = 16;
        ctx.lineCap = "round";
        ctx.lineJoin = "round";
        ctx.strokeStyle = "black";

        clearCanvas();
    });
</script>

<div class="drawing-container">
    <canvas
        bind:this={canvas}
        width={SIZE}
        height={SIZE}
        onpointerdown={startDrawing}
        onpointermove={draw}
        onpointerup={stopDrawing}
        onpointercancel={stopDrawing}
    ></canvas>

    <div class="buttons">
        <button onclick={clearCanvas}>Clear</button>
        <button onclick={() => console.log(getPixelData())}>Get Pixels</button>
        <button onclick={() => {
            create_fourier_series_lettering(getPixelData(), new Uint8Array([1, 2, 4] /* test */))
        }}>Create</button>
    </div>
</div>

<style>
    .drawing-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
    }

    canvas {
        width: 90vmin;
        height: 90vmin;
        border: 1px solid #888;
        background: white;
        touch-action: none;
    }

    .buttons {
        display: flex;
        gap: 8px;
    }
</style>
