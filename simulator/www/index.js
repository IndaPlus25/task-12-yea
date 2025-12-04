
import { Universe, Cell } from "./pkg/simulator.js";

// Constants for the color of the cells and grid
const CELL_SIZE = 7; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000"; 

let wasmMemory;

async function run() {
  // importing memory (You don't know how hard this was after following that outdated guide)
  const wasmModule = await import("./pkg/simulator_bg.wasm");
  wasmMemory = wasmModule.memory;

  // make an instance of universe and get it's measurements
  const universe = Universe.new();
  const width = universe.width();
  const height = universe.height();
  
  // get the canvas in index.html and fix the canvas according to specs
  const canvas = document.getElementById("simulator-canvas");
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const ctx = canvas.getContext('2d');

  const getIndex = (row, column) => {
    return row * width + column;
  };    
  
  // Draw the grid itself
  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  };

  // Draw the cells, inside the grid
  const drawCells = () => {
    // get the pointer for cells and read from memory
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(wasmMemory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    // draw each cell itself based on if it's dead or not
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = cells[idx] === Cell.Dead
          ? DEAD_COLOR
          : ALIVE_COLOR;

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke(); 
  };

  // Loop for calling the drawing functions and calculating the next generation of cells
  const renderLoop = () => {
      universe.tick();
      
      drawGrid();
      drawCells();
    
      requestAnimationFrame(renderLoop);
  };

  // Call the functions at least one to kickstart everything
  drawGrid();
  drawCells();
  renderLoop();
} 

// run all of the code above/initialize everything
run();
