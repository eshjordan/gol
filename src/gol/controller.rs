use super::grid::Grid;
use super::cell::{CellState, Cell};
use pixels::{Pixels, SurfaceTexture, Error};
use winit::dpi::PhysicalSize;
use winit::window::Window;


const GRID_ROWS: usize = 64;
const GRID_COLS: usize = 64;
const GRID_CELL_PIXELS: usize = 10;
const PIXEL_ROWS: usize = GRID_ROWS * GRID_CELL_PIXELS;
const PIXEL_COLS: usize = GRID_COLS * GRID_CELL_PIXELS;


pub struct Controller {
    paused: bool,
    pixels: Pixels,
    calculation_grid: Grid,
}

impl Controller {
    pub fn new(window: &Window) -> Controller {

        let pixels = {
            window.set_inner_size(PhysicalSize::new(PIXEL_COLS as u32, PIXEL_ROWS as u32));
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(PIXEL_COLS as u32, PIXEL_ROWS as u32, surface_texture).unwrap()
        };

        let calculation_grid = Grid::new(GRID_ROWS, GRID_COLS);

        Controller { paused: true, pixels, calculation_grid}
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }

    pub fn resize_window(&mut self, size: PhysicalSize<u32>) -> Result<(), pixels::TextureError> {
        self.pixels.resize_surface(size.width, size.height)
    }

    pub fn initialise_game(&mut self) -> Grid {
        let filled_cells = [(2,2), (2,3), (2,4)];
        for (row, col) in filled_cells {
            self.calculation_grid.set_cell_at(row, col, CellState::Alive);
        }

        self.calculation_grid.clone()
    }

    pub fn generate(&mut self, display_grid: &mut Grid) {

        if self.paused {
            return
        }

        self.calculation_grid.kill_all();

        let mut neighbours: Vec<&Cell> = vec![];
        for row in 0..GRID_ROWS {
            for col in 0..GRID_COLS {
                let cell = display_grid.cell_at(row, col);
                display_grid.cell_neighbours_at(row, col, &mut neighbours);
                let living_neighbours = neighbours.iter().filter(|cell| cell.living()).count();

                let new_cell_state = {
                    if cell.living() && (living_neighbours < 2 || living_neighbours > 3) {
                        // Underpopulation / Overpopulation
                        CellState::Dead
                    } else if cell.living() && (living_neighbours == 2 || living_neighbours == 3) {
                        // Survives
                        CellState::Alive
                    } else if !cell.living() && living_neighbours == 3 {
                        // New cell birth
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                };

                self.calculation_grid.set_cell_at(row, col, new_cell_state);
            }
        }

        display_grid.kill_all();

        let living_cells = self.calculation_grid.iter().filter(|(row, col, cell)| cell.living());

        for (row, col, _) in living_cells {
            display_grid.set_cell_at(row, col, CellState::Alive);
        }

    }

    pub fn manual_set(&self, grid: &mut Grid, x: f32, y: f32) {
        let row = (y / GRID_CELL_PIXELS as f32).floor() as usize;
        let col = (x / GRID_CELL_PIXELS as f32).floor() as usize;
        grid.set_cell_at(row, col, CellState::Alive);
    }

    pub fn manual_unset(&self, grid: &mut Grid, x: f32, y: f32) {
        let row = (y / GRID_CELL_PIXELS as f32).floor() as usize;
        let col = (x / GRID_CELL_PIXELS as f32).floor() as usize;
        grid.set_cell_at(row, col, CellState::Dead);
    }

    pub fn display(&mut self, grid: &Grid) -> Result<(), pixels::Error> {
        let frame = self.pixels.frame_mut();
        frame.fill(0x00);

        let living_cells = grid.iter().filter(|(row, col, cell)| cell.living());

        for (grid_row, grid_col, _) in living_cells {
            let pixel_row_grid_col_iter = frame.chunks_exact_mut(4*GRID_CELL_PIXELS);

            let pixel_row_at_row_col = pixel_row_grid_col_iter
                .skip(grid_col) // Get to the first set of pixels in the first row, correct column
                .step_by(GRID_COLS) // Every iteration, move down 1 row of pixels
                .skip(GRID_CELL_PIXELS * grid_row) // Get to the first set of pixels in the correct row, correct column
                .take(GRID_CELL_PIXELS); // Only iterate over the correct number of pixel rows in the grid cell

            for row_pixels in pixel_row_at_row_col {
                row_pixels.fill(0xFF);

                const RedOffset: usize = 0;
                const GreenOffset: usize = 1;
                const BlueOffset: usize = 2;
                const AlphaOffset: usize = 3;

                struct RGBA {
                    R: u8,
                    G: u8,
                    B: u8,
                    A: u8,
                }

                const WHITE: RGBA = RGBA{R: 255, G: 255, B: 255, A: 255};
                const BLACK: RGBA = RGBA{R: 0, G: 0, B: 0, A: 255};
                const RED: RGBA = RGBA{R: 255, G: 0, B: 0, A: 255};
                const GREEN: RGBA = RGBA{R: 0, G: 255, B: 0, A: 255};
                const BLUE: RGBA = RGBA{R: 0, G: 0, B: 255, A: 255};

                let cell_colour = RED;

                for pixel_r in row_pixels.iter_mut().skip(RedOffset).step_by(4) {
                    *pixel_r = cell_colour.R;
                }

                for pixel_r in row_pixels.iter_mut().skip(GreenOffset).step_by(4) {
                    *pixel_r = cell_colour.G;
                }

                for pixel_r in row_pixels.iter_mut().skip(BlueOffset).step_by(4) {
                    *pixel_r = cell_colour.B;
                }

                for pixel_r in row_pixels.iter_mut().skip(AlphaOffset).step_by(4) {
                    *pixel_r = cell_colour.A;
                }
            }
        }

        self.pixels.render()
    }

}
