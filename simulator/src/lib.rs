use js_sys::Math;

use wasm_bindgen::prelude::*;

// const so I don't have to write Cell::x a million times
const ALIVE: Cell = Cell::Alive;
const DEAD: Cell = Cell::Dead;

// Const for a 2x2 shape from the angle of the middle cell
const BLOCK: [Cell; 9] = [
    ALIVE, ALIVE, DEAD,
    ALIVE, ALIVE, DEAD,
    DEAD,  DEAD,  DEAD,
];

// Const for a 1x3 shape 
const OSCILLATOR: [Cell; 9] = [
    DEAD, ALIVE, DEAD,
    DEAD, ALIVE, DEAD,
    DEAD,  ALIVE,  DEAD,
];

// A cell is either dead or alive
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// Universe is the map
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// Functions for universe
#[wasm_bindgen]
impl Universe {
    // Getters
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row*self.width + column) as usize
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    // Counts how many live cells there are next to a specified cell by row and column
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        // check three cells next to eachother, then moves back and down one row and checks three cells next to eachother etc
        // Ignores the cell straight in the middle because that is the specified cell, it's not a neighbor to itself
        // Simulates wrapping around the world by using modulo, which makes the world a donut (torus)
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    // returns a list of all of the nearby cells to a specified cell, including itself
    // made this so I can get rid of those pesky shapes that never change
    // works the same way as live_neighbor_count
    fn get_area(&self, row: u32, column: u32) -> Vec<Cell> {
        let mut area: Vec<Cell> = vec![DEAD; 9];
        let mut i = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                area[i] = self.cells[idx];
                i+=1;
            }
        }
        area
    }

    // function for calculating the next generation of cells based on the current one
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        // loops around for every single cell in Universe
        for row in 0..self.height {
            for column in 0..self.width {
                // get the current cells id and how many live neighbors it has
                let id = self.get_index(row, column);
                let cell = self.cells[id];
                let neighbors = self.live_neighbor_count(row, column);

                // Cells die if they have less than 2 or more than 3 neighbors
                // Dead cells become alive if they are near exactly 3 neighbors
                // made a little tweak where they had 1% chance of surviving overpopulation, 
                // but decided it was too much after getting rid of the squares and oscillators
                let next_cell = match (cell, neighbors) {
                    (ALIVE, count) if count < 2 => DEAD,
                    (ALIVE, 2) | (ALIVE, 3) => ALIVE,
                    // (ALIVE, count) if count > 3 => {if Math::random() < 0.99 {DEAD} else {cell}},
                    (ALIVE, count) if count > 3 => DEAD,
                    (DEAD, 3) => ALIVE,
                    (otherwise, _) => otherwise,
                };

                let area = self.get_area(row, column);
                
                // If there is a 2x2 square of cells, kill the cell in the bottom right of it (the current cell), 
                // kill the cell above it and make a cell above that and to the left alive, making it into an oscillator
                // after the algorithm handles the rest
                if area == BLOCK {
                    next[id] = DEAD;
                    next[self.get_index((row - 1) % self.height, column)] = DEAD;
                    next[self.get_index((row - 2) % self.height, (column - 1) % self.width)] = ALIVE;
                // If there is a 1x3 square of cells, make the cell above the current one alive,
                // the algorithm handles the rest and makes it explode into a 2x3 of alive cells
                // How it looked this cycle    How it would have looked without intervention     how it looks now
                // DEAD, ALIVE, DEAD,          DEAD, DEAD, DEAD,                                 DEAD, ALIVE, DEAD,             ALIVE, ALIVE, ALIVE,     
                // DEAD, ALIVE, DEAD,          ALIVE, ALIVE, ALIVE,                              ALIVE, ALIVE, ALIVE, ->        ALIVE, ALIVE, ALIVE,
                // DEAD,  ALIVE,  DEAD,        DEAD, DEAD, DEAD,                                 DEAD, DEAD, DEAD,              DEAD, DEAD, DEAD,
                } else if area == OSCILLATOR {
                    next[self.get_index((row - 1) % self.height, column)] = ALIVE;
                }

                next[id] = next_cell;
            }
        }
        self.cells = next;
    }

    // constructor
    pub fn new() -> Universe{
        let width = 64;
        let height = 64;

        let mut cells: Vec<Cell> = vec![DEAD; width*height];

        // Each cell has a 57% chance of being alive when the simulation starts
        for cell in cells.iter_mut() {
            if Math::random() < 0.57 {
                *cell = ALIVE;
            }
        }

        Universe { width: width as u32, height: height as u32, cells: cells }
    }
}