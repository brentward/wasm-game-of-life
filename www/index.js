import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

main();

function main() {
    // const CELL_SIZE = 5; // px
    const GRID_COLOR = "#CCCCCC";
    const DEAD_COLOR = "#FFFFFF";
    const ALIVE_COLOR = "#000000";

    const universe = Universe.new();
    const width = universe.width();
    const height = universe.height();
    const CELL_SIZE = universe.size();

    const canvas = document.getElementById("game-of-life-canvas");
    // const ctx = canvas.getContext('2d');

    // canvas.height = (CELL_SIZE + 1) * height + 1;
    // canvas.width = (CELL_SIZE + 1) * width + 1;
    canvas.addEventListener("click", event => {
        const insertPopulation = document.getElementById("insert").value;
        const hFlip = document.getElementById("h-flip").checked;
        const vFlip = document.getElementById("v-flip").checked;
        const invert = document.getElementById("invert").checked;

        const boundingRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasTop = (event.clientY - boundingRect.top) * scaleY;

        const row = (Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1) + height) % height;
        const col = (Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1) + width) % width;

        if (insertPopulation === "toggle") {
            universe.toggle_cell(row, col);
        } else {
            universe.seed_population(height - row, col, insertPopulation, hFlip, vFlip, invert);
        }

        universe.render()
        // drawGrid();
        // drawCells();
    });

    let animationId = null;

    const renderLoop = () => {
        fps.render();

        // for (let i = 0; i < 9; i++) {
        universe.tick();
        universe.render();
        // }
        // drawGrid();
        // drawCells();

        animationId = requestAnimationFrame(renderLoop);
    };

    const isPaused = () => {
        return animationId === null;
    };

    const playPauseButton = document.getElementById("play-pause");
    const destroyAllLife = document.getElementById("destroy-all-life");
    const randomPopulation = document.getElementById("random-population");
    playPauseButton.textContent = "❚❚";
    destroyAllLife.textContent = "Destroy All Life";
    randomPopulation.textContent = "Random Population";

    const play = () => {
        playPauseButton.textContent = "❚❚";
        renderLoop();
    };

    const pause = () => {
        playPauseButton.textContent = "▶";
        cancelAnimationFrame(animationId);
        animationId = null;
    };

    playPauseButton.addEventListener("click", event => {
        if (isPaused()) {
            play();
        } else {
            pause();
        }
    });

    destroyAllLife.addEventListener("click", event => {
        universe.destroy_all_life();
        universe.render();
        // drawGrid();
        // drawCells();
    });

    randomPopulation.addEventListener("click", event => {
        universe.random_population();
        universe.render();
        // drawGrid();
        // drawCells();
    });

    // const drawGrid = () => {
    //     drawScene(gl, programInfo, buffers)
    //
    //     // ctx.beginPath();
    //     // ctx.strokeStyle = GRID_COLOR;
    //
    //     // Vertical lines
    //     for (let i = 0; i <= width; i++) {
    //         // ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    //         // ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    //     }
    //
    //     // Horizontal lines
    //     for (let j = 0; j <= width; j++) {
    //         // ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    //         // ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    //     }
    //
    //     // ctx.stroke();
    // }

    const getIndex = (row, column) => {
        return row * width + column
    };

    // const drawCells = () => {
    //     const cellsPtr = universe.cells();
    //     const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
    //
    //     // ctx.beginPath();
    //     //
    //     // ctx.fillStyle = ALIVE_COLOR;
    //     for (let row = 0; row < height; row++) {
    //         for (let col = 0; col < width; col++) {
    //             const idx = getIndex(row, col);
    //             if (cells[idx] !== Cell.Alive) {
    //                 continue;
    //             }
    //
    //             // ctx.fillStyle = cells[idx] === Cell.Dead
    //             //     ? DEAD_COLOR
    //             //     : ALIVE_COLOR;
    //             //
    //             // ctx.fillRect(
    //             //     col * (CELL_SIZE + 1) + 1,
    //             //     row * (CELL_SIZE + 1) + 1,
    //             //     CELL_SIZE,
    //             //     CELL_SIZE
    //             // );
    //         }
    //     }
    //
    //     // ctx.fillStyle = DEAD_COLOR;
    //     for (let row = 0; row < height; row++) {
    //         for (let col = 0; col < width; col++) {
    //             const idx = getIndex(row, col);
    //             if (cells[idx] !== Cell.Dead) {
    //                 continue;
    //             }
    //
    //             // ctx.fillStyle = cells[idx] === Cell.Dead
    //             //     ? DEAD_COLOR
    //             //     : ALIVE_COLOR;
    //             //
    //             // ctx.fillRect(
    //             //     col * (CELL_SIZE + 1) + 1,
    //             //     row * (CELL_SIZE + 1) + 1,
    //             //     CELL_SIZE,
    //             //     CELL_SIZE
    //             // );
    //         }
    //     }
    //
    //     // ctx.stroke();
    // }
    universe.render();
    // drawGrid();
    // drawCells();

    requestAnimationFrame(renderLoop);

}

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;
        this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
}
