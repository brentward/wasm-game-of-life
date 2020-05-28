mod utils;
mod render;

use std::vec;

use wasm_bindgen::prelude::*;
use js_sys::Math;
use web_sys::{console, WebGlProgram};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(not(test))]
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(test)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        print!("[TEST LOG] ");
        &println!( $( $t )* );
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        #[cfg(not(test))]
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        #[cfg(not(test))]
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Population {
    height: u32,
    width: u32,
    cells: Vec<(u32, u32)>,
}

impl Population {
    pub fn new(name: String) -> Population {
        let (cells, height, width) = match name.as_str() {
            "block" => (vec![
                (0, 0), (0,1),
                (1, 0), (1, 1)
            ], 2, 2),
            "blinker" => (vec![
                (0, 0), (0, 1), (0, 2)
            ], 1, 3),
            "toad" => (vec![
                        (0, 1), (0, 2), (0,3),
                (1, 0), (1, 1), (1, 2)
            ], 2, 4),
            "beacon" => (vec![
                (0, 0), (0, 1),
                (1, 0),
                                (2, 3),
                        (3, 2), (3, 3)
            ], 4, 4),
            "pulsar" => (vec![
                                (0, 2), (0, 3), (0, 4),                         (0, 8), (0, 9), (0, 10),
                (2, 0),                                 (2, 5),         (2, 7),                                 (2, 12),
                (3 ,0),                                 (3, 5),         (3, 7),                                 (3, 12),
                (4, 0),                                 (4, 5),         (4, 7),                                 (4, 12),
                                (5, 2), (5, 3), (5, 4),                         (5, 8), (5, 9), (5, 10),
                                (7, 2), (7, 3), (7, 4),                         (7, 8), (7, 9), (7, 10),
                (8, 0),                                 (8, 5),         (8, 7),                                 (8, 12),
                (9, 0),                                 (9, 5),         (9, 7),                                 (9, 12),
                (10,0),                                 (10,5),         (10,7),                                 (10,12),
                                (12,2), (12,3), (12,4),                         (12,8), (12,9), (12,10)
            ], 13, 13),
            "i-column" => (vec![
                (0, 0), (0, 1), (0, 2),
                        (1, 1),
                        (2, 1),
                (3, 0), (3, 1), (3, 2),

                (5, 0), (5, 1), (5, 2),
                (6, 0), (6, 1), (6, 2),

                (8, 0), (8, 1), (8, 2),
                        (9, 1),
                        (10,1),
                (11,0), (11,1), (11,2)
            ], 12, 3),
            "glider" => (vec![
                        (0, 1),
                                (1, 2),
                (2, 0), (2, 1), (2, 2)
            ], 3, 3),
            "lwss" => (vec![
                        (0, 1), (0, 2), (0, 3), (0, 4),
                (1, 0),                         (1, 4),
                                                (2, 4),
                (3, 0),                 (3, 3)
            ], 4, 5),
            "mwss" => (vec![
                        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5),
                (1, 0),                                 (1, 5),
                                                        (2, 5),
                (3, 0),                         (3, 4),
                                (4, 2)
            ], 5, 6),
            "hwss" => (vec![
                        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),
                (1, 0),                                         (1, 6),
                                                                (2, 6),
                (3, 0),                                 (3, 5),
                                (4, 2), (4, 3)
            ], 5, 7),
            _ => (vec![
                (0, 0), (0,1),
                (1, 0), (1, 1)
            ], 2, 2),
        };
        Population {
            height,
            width,
            cells
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    size: u8,
    cells: [Vec<Cell>; 2],
    cells_idx: usize,
    next_cells_idx: usize,
    cell_program: Option<WebGlProgram>,
    grid_program: Option<WebGlProgram>,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if col == 0 {
            self.width - 1
        } else {
            col - 1
        };

        let east = if col == self.width - 1 {
            0
        } else {
            col + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[self.cells_idx][nw] as u8;

        let n = self.get_index(north, col);
        count += self.cells[self.cells_idx][n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[self.cells_idx][ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[self.cells_idx][w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[self.cells_idx][e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[self.cells_idx][sw] as u8;

        let s = self.get_index(south, col);
        count += self.cells[self.cells_idx][s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[self.cells_idx][se] as u8;

        count
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells[self.cells_idx]
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row % self.height, col % self.width);
            self.cells[self.cells_idx][idx] = Cell::Alive;
        }
    }

    fn clear_cells(&mut self, row: u32, col: u32, h_size: u32, v_size: u32) {
        for row in row..row + v_size {
            for col in col..col + h_size {
                let idx = self.get_index(row % self.height, col % self.width);
                self.cells[self.cells_idx][idx] = Cell::Dead;
            }
        }
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 192;
        let height = 128;
        let size = 5;

        let (cell_program, grid_program) = if !cfg!(test) {
            match render::start((size as u32 + 1) * width + 1, (size as u32 + 1) * height + 1) {
                Ok(program_option_tup) => program_option_tup,
                Err(_) => (None, None),
            }
        } else {
            (None, None)
        };


        // render::set_canvas_size(((size as u32 + 1) * height + 1) * 4, ((size as u32 + 1) * width + 1) * 2)
        //     .expect("Universe::new() calling render::set_canvas_size() failed");
        let cells_0: Vec<Cell> = (0..width * height).map(|_i| Cell::Dead).collect();
        let cells_1: Vec<Cell> = (0..width * height).map(|_i| Cell::Dead).collect();
        let cells = [cells_0, cells_1];
        Universe {
            width,
            height,
            cells,
            size,
            cells_idx: 0,
            next_cells_idx: 1,
            cell_program,
            grid_program,
        }
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick()");

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[self.cells_idx][idx];
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
                        // log!("cel[{}, {}] died of loneliness", row, col);
                        Cell::Dead
                    },
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => {
                        // log!("cel[{}, {}] died of agoraphobia", row, col);
                        Cell::Dead
                    },
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => {
                        // log!("cel[{}, {}] was born", row, col);
                        Cell::Alive
                    },
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                self.cells[self.next_cells_idx][idx] = next_cell
            }
        }

        self.cells_idx = (self.cells_idx + 1) & 1;
        self.next_cells_idx = (self.next_cells_idx + 1) & 1;
    }

    pub fn render(&self) {
        let mut vertices: Vec<f32> = vec![];
        let mut grid_vertices: Vec<f32> = vec![];
        let x_pixels = (self.size as u32 + 1) * self.width + 1;
        let y_pixels = (self.size as u32 + 1) * self.height + 1;
        // let z = 0.0;
        let x_start = -1.0;
        let y_start = -1.0;
        let x_grid = 2.0 / x_pixels as f32;
        let x_size = x_grid * self.size as f32;
        let y_grid =  2.0 / y_pixels as f32;
        let y_size = y_grid * self.size as f32;
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_index(y, x);
                if self.cells[self.cells_idx][idx] == Cell::Alive {
                    let cell_x0 = x_start + (x as f32 * (x_grid + x_size)) +  x_grid;
                    let cell_y0 = y_start + (y as f32 * (y_grid + y_size)) +  y_grid;
                    let cell_x1 = cell_x0 + x_size;
                    let cell_y1 = cell_y0 + y_size;
                    vertices.push(cell_x0);
                    vertices.push(cell_y0);
                    vertices.push(0.0);
                    vertices.push(cell_x1);
                    vertices.push(cell_y0);
                    vertices.push(0.0);
                    vertices.push(cell_x0);
                    vertices.push(cell_y1);
                    vertices.push(0.0);
                    vertices.push(cell_x1);
                    vertices.push(cell_y1);
                    vertices.push(0.0);
                    vertices.push(cell_x0);
                    vertices.push(cell_y1);
                    vertices.push(0.0);
                    vertices.push(cell_x1);
                    vertices.push(cell_y0);
                    vertices.push(0.0);
                    
                }
                if x == 0 {
                    let line_y = y_start + (y as f32 * (y_grid + y_size)) + (y_grid / 2.0);
                    grid_vertices.push(-1.0);
                    grid_vertices.push(line_y);
                    grid_vertices.push(0.0);
                    grid_vertices.push(1.0);
                    grid_vertices.push(line_y);
                    grid_vertices.push(0.0);
                }
            }
            let line_x = x_start + (x as f32 * (x_grid + x_size)) + (x_grid / 2.0);
            grid_vertices.push(line_x);
            grid_vertices.push(-1.0);
            grid_vertices.push(0.0);
            grid_vertices.push(line_x);
            grid_vertices.push(1.0);
            grid_vertices.push(0.0);

        }
        grid_vertices.push(-1.0 - (x_grid / 2.0));
        grid_vertices.push(1.0 - (y_grid / 2.0));
        grid_vertices.push(0.0);
        grid_vertices.push(1.0 - (x_grid / 2.0));
        grid_vertices.push(1.0 - (y_grid / 2.0));
        grid_vertices.push(0.0);
        grid_vertices.push(1.0 - (x_grid / 2.0));
        grid_vertices.push(-1.0 - (y_grid / 2.0));
        grid_vertices.push(0.0);
        grid_vertices.push(1.0 - (x_grid / 2.0));
        grid_vertices.push(1.0 - (y_grid / 2.0));
        grid_vertices.push(0.0);

        let cell_program = match &self.cell_program {
            Some(program) => program,
            None => return (),
        };

        let grid_program = match &self.grid_program {
            Some(program) => program,
            None => return (),
        };

        render::render(cell_program, vertices, grid_program, grid_vertices).expect("error rendering");
    }

    pub fn render_to_string(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn cells(&self) -> *const Cell {
        self.cells[self.cells_idx].as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells[0] = (0..width * self.height).map(|_i| Cell::Dead).collect();
        self.cells[1] = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells[0] = (0..self.width * height).map(|_i| Cell::Dead).collect();
        self.cells[1] = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size as u8;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[self.cells_idx][idx].toggle();
    }

    pub fn seed_population(&mut self, row: u32, col: u32, pop_name: String, h_flip: bool, v_flip: bool, invert: bool) {
        // log!(
        //     "Universe::seed_population() row: {}, col: {}, name: {}, h_flip: {}, v_flip: {}, invert: {}",
        //     row,
        //     col,
        //     pop_name,
        //     h_flip,
        //     v_flip,
        //     invert
        // );
        let pop = Population::new(pop_name);
        let (height, width) = if invert {
            (pop.width, pop.height)
        } else {
            (pop.height, pop.width)
        };
        let row = (self.height + row) - (height / 2);
        let col = (self.width + col) - (width / 2);
        // log!("Universe::seed_population() adjusted row: {}, col: {}", row, col);
        let mut cells = Vec::new();
        for (cell_y, cell_x) in pop.cells {
            let (cell_row, cell_col) = if invert {
                (cell_x, cell_y)
            } else {
                (cell_y, cell_x)
            };
            let row = if v_flip {
                row + height - cell_row
            } else {
                row + cell_row
            };
            let col = if h_flip {
                col + width - cell_col
            } else {
                col + cell_col
            };
            cells.push((row, col));
        }
        self.clear_cells(row, col, width, height);
        self.set_cells(cells.as_slice());
    }

    pub fn destroy_all_life(&mut self) {
        self.clear_cells(0, 0, self.width, self.height);
    }

    pub fn random_population(&mut self) {
        self.clear_cells(0, 0, self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                self.cells[self.cells_idx][idx] = {
                    if Math::random() < 0.5 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                };
            }
        }
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in (self.cells[self.cells_idx]).as_slice().chunks(self.width as usize) {
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
        println!("{}", input_universe.render_to_string());
        input_universe.tick();
        println!("\ninput universe:");
        println!("{}", input_universe.render_to_string());
        println!("\nexpected universe:");
        println!("{}", expected_universe.render_to_string());
        assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
    }
}
