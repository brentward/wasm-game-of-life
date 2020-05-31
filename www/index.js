import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

main();

function main() {
    const universe = Universe.new();
    let width = universe.width();
    let height = universe.height();
    let cellSize = universe.size();

    const canvas = document.getElementById("game-of-life-canvas");
    const playPauseButton = document.getElementById("play-pause");
    const universeSpeed = document.getElementById("universeSpeed");
    const universeSpeedValue = document.getElementById("universeSpeedValue");

    const destroyAllLife = document.getElementById("destroy-all-life");
    const randomPopulation = document.getElementById("random-population");
    const populationDensitySlider = document.getElementById("populationDensity");
    const populationDensityValue = document.getElementById("populationDensityValue");
    const gridActionToggle = document.getElementById("toggle");
    const gridActionInsertPopulation = document.getElementById("insertPopulation");
    const insertPopulation = document.getElementById("insert");
    const hFlip = document.getElementById("h-flip");
    const vFlip = document.getElementById("v-flip");
    const invert = document.getElementById("invert");
    const hFlipLabel = document.getElementById("hFlipLabel");
    const vFlipLabel = document.getElementById("vFlipLabel");
    const invertLabel = document.getElementById("invertLabel");
    const resizeUniverse = document.getElementById('resize-universe');
    const hSizeSet = document.getElementById("hSize");
    const vSizeSet = document.getElementById("vSize");
    const cellSizeSet = document.getElementById("cellSize");
    let speedDown = false;
    let speedUp = false;
    let speedDownFactor = 1;
    let speedUpFactor = 1;
    let speedDownCounter = 1;

    gridActionToggle.checked = true;
    insertPopulation.disabled = true;
    hFlip.disabled = true;
    vFlip.disabled = true;
    invert.disabled = true;
    hFlipLabel.style.color = "#aaa";
    vFlipLabel.style.color = "#aaa";
    invertLabel.style.color = "#aaa";
    populationDensityValue.innerHTML = populationDensitySlider.value;
    universeSpeedValue.innerHTML = "normal";
    resizeUniverse.textContent = "Resize"
    hSizeSet.valueAsNumber = width;
    hSizeSet.size = 4;
    vSizeSet.valueAsNumber = height;
    vSizeSet.size = 4;
    cellSizeSet.valueAsNumber = cellSize;
    cellSizeSet.size = 4;



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

        if (gridActionToggle.checked) {
            universe.toggle_cell(row, col);
        } else {
            universe.seed_population(row, col, insertPopulation, hFlip, vFlip, invert);
        }

        universe.render()
    });

    let animationId = null;

    const renderLoop = () => {
        if (speedUp) {
            for (let i = 0; i < speedUpFactor; i++) {
                universe.tick();
            }
        } else if (speedDown) {
            if (speedDownCounter === speedDownFactor) {
                speedDownCounter = 1;
                universe.tick();
            } else {
                speedDownCounter++;
            }
        } else {
            universe.tick();
        }

        universe.render();
        fps.render();

        animationId = requestAnimationFrame(renderLoop);
    };

    const isPaused = () => {
        return animationId === null;
    };


    populationDensitySlider.oninput = function() {
        populationDensityValue.innerHTML = this.valueAsNumber;
    }

    universeSpeed.oninput = function() {
        if (this.valueAsNumber >= 56) {
            speedUpFactor = Math.ceil((this.valueAsNumber - 50) / 50 * 10);
            speedDown = false;
            speedUp = true;
            universeSpeedValue.innerHTML = speedUpFactor + "x normal";
        } else if (this.valueAsNumber <= 45) {
            speedDownFactor = Math.ceil(10.0 - (this.valueAsNumber / 50) * 10);
            speedDownCounter = speedDownFactor;
            speedDown = true;
            speedUp = false;
            universeSpeedValue.innerHTML = "1/" + speedDownFactor + " normal";

        } else {
            universeSpeedValue.innerHTML = "normal";
            speedDown = false;
            speedUp = false;
        }
    }


    playPauseButton.textContent = "▶";
    playPauseButton.style.width = "125px";
    playPauseButton.style.height = "50px";
    destroyAllLife.textContent = "Destroy All Life";
    randomPopulation.textContent = "Seed random cells";

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
        universe.random_population(populationDensitySlider.valueAsNumber / 100);
        universe.render();
    });

    resizeUniverse.addEventListener("click", event => {
        if (isNaN(vSizeSet.valueAsNumber)) {
            vSizeSet.valueAsNumber = height;
        } else if (vSizeSet.valueAsNumber < 1) {
            vSizeSet.valueAsNumber = 1;
        } else if (vSizeSet.valueAsNumber > 768 ) {
            vSizeSet.valueAsNumber = 768;
        }
        if (isNaN(hSizeSet.valueAsNumber)) {
            hSizeSet.valueAsNumber = height;
        } if (hSizeSet.valueAsNumber < 1) {
            // debugger;
            hSizeSet.valueAsNumber = 1;
        } else if (hSizeSet.valueAsNumber > 768 ) {
            hSizeSet.valueAsNumber = 768;
        }
        if (isNaN(cellSizeSet.valueAsNumber)) {
            cellSizeSet.valueAsNumber = cellSize;
        } if (cellSizeSet.valueAsNumber < 1) {
            cellSizeSet.valueAsNumber = 1;
        } else if (cellSizeSet.valueAsNumber > 255) {
            cellSizeSet.valueAsNumber = 255
        } else if (((cellSizeSet.valueAsNumber + 1) * hSizeSet.valueAsNumber + 1) * ((cellSizeSet.valueAsNumber + 1) * vSizeSet.valueAsNumber + 1) > 16810000 ) {
            vSizeSet.valueAsNumber = height;
            hSizeSet.valueAsNumber = width;
            cellSizeSet.valueAsNumber = cellSize;
        }
        height = vSizeSet.valueAsNumber;
        width = hSizeSet.valueAsNumber;
        cellSize = cellSizeSet.valueAsNumber;
        universe.resize(width, height, cellSize);
        universe.render();
    });

    gridActionInsertPopulation.addEventListener("click", event => {
        insertPopulation.disabled = false;
        hFlip.disabled = false;
        vFlip.disabled = false;
        invert.disabled = false;
        hFlipLabel.style.color = "#000";
        vFlipLabel.style.color = "#000";
        invertLabel.style.color = "#000";

    });

    gridActionToggle.addEventListener("click", event => {
        insertPopulation.disabled = true;
        hFlip.disabled = true;
        vFlip.disabled = true;
        invert.disabled = true;
        hFlipLabel.style.color = "#aaa";
        vFlipLabel.style.color = "#aaa";
        invertLabel.style.color = "#aaa";
    });


    universe.render();

    // requestAnimationFrame(renderLoop);

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
        if (this.frames.length > 30) {
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
        this.fps.textContent = `fps = ${Math.round(mean)}`.trim();
    }
}
