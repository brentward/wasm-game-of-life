import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

main();

function main() {
    const universe = Universe.new();
    const width = universe.width();
    const height = universe.height();
    const cellSize = universe.size();

    const canvas = document.getElementById("game-of-life-canvas");

    canvas.addEventListener("click", event => {
        const insertPopulation = document.getElementById("insert").value;
        const hFlip = document.getElementById("h-flip").checked;
        const vFlip = document.getElementById("v-flip").checked;
        const invert = document.getElementById("invert").checked;

        const boundingRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasBottom = (boundingRect.bottom - event.clientY) * scaleY;

        const row = (Math.min(Math.floor(canvasBottom / (cellSize + 1)), height - 1));
        const col = (Math.min(Math.floor(canvasLeft / (cellSize + 1)), width - 1));

        if (insertPopulation === "toggle") {
            universe.toggle_cell(row, col);
        } else {
            universe.seed_population(row, col, insertPopulation, hFlip, vFlip, invert);
        }

        universe.render()
    });

    let animationId = null;

    const renderLoop = () => {
        fps.render();

        // for (let i = 0; i < 9; i++) {
        universe.tick();
        universe.render();
        // }

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
    });

    randomPopulation.addEventListener("click", event => {
        universe.random_population();
        universe.render();
    });

    universe.render();

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
