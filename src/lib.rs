mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Math;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

#[cfg(not(test))]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(test)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        print!("[TEST LOG] ");
        &println!( $( $t )* );
    }
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                //
                // log!(
                //     "cel[{}, {}] is initially {:?} and has {} life neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                // );

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => {
                        log!("cel[{}, {}] died of loneliness", row, col);
                        Cell::Dead
                    },
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => {
                        log!("cel[{}, {}] died of agoraphobia", row, col);
                        Cell::Dead
                    },
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => {
                        log!("cel[{}, {}] was born", row, col);
                        Cell::Alive
                    },
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        // let cells = (0..width * height)
        //     .map(|_i| {
        //         if Math::random() < 0.5 {
        //             Cell::Alive
        //         } else {
        //             Cell::Dead
        //         }
        //     })
        //     .collect();
        let cells = (0..width * height).map(|_i| Cell::Dead).collect();

        let mut universe = Universe {
            width,
            height,
            cells,
        };
        universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
        universe

    }

    pub fn render(&self) -> String {
        self.to_string()
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

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spaceship() {
        let mut input_universe = Universe::new();
        input_universe.set_width(6);
        input_universe.set_height(6);
        input_universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);


        let mut expected_universe = Universe::new();
        expected_universe.set_width(6);
        expected_universe.set_height(6);
        expected_universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);

        println!("input universe before tick:");
        println!("{}", input_universe.render());
        input_universe.tick();
        println!("\ninput universe:");
        println!("{}", input_universe.render());
        println!("\nexpected universe:");
        println!("{}", expected_universe.render());
        assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
    }
}
