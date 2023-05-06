use std::cmp::{max, min};

use super::cell::{Cell, CellState};

#[derive(Clone)]
pub struct Grid {
    cells: Vec<Cell>,
    rows: usize,
    cols: usize,
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    idx: usize
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (usize, usize, &'a Cell);

    fn next(&mut self) -> Option<(usize, usize, &'a Cell)> {
        let mut cell = self.grid.cells.get(self.idx)?;

        let row = self.idx/self.grid.cols;
        let col = self.idx%self.grid.cols;
        self.idx += 1;
        Some((row, col, cell))
    }
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Grid {
        Grid {
            cells: vec![Cell { state: CellState::Dead }; rows*cols],
            rows,
            cols,
        }
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator{grid: &self, idx: 0}
    }

    pub fn cell_at(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row*self.cols + col]
    }

    pub fn set_cell_at(&mut self, row: usize, col: usize, state: CellState) {
        self.cells[row*self.cols + col].state = state;
    }

    pub fn cell_neighbours_at<'a, 'b>(&'a self, row: usize, col: usize, neighbours: &'b mut Vec<&'a Cell>) {
        neighbours.clear();

        let prev_row = row.checked_sub(1).unwrap_or_default();
        let next_row = min(self.rows-1, row+1);
        let prev_col = col.checked_sub(1).unwrap_or_default();
        let next_col = min(self.cols-1, col+1);

        for neighbour_row in prev_row..=next_row {
            for neighbour_col in prev_col..=next_col {
                if neighbour_row == row && neighbour_col == col {
                    continue;
                }
                neighbours.push(self.cell_at(neighbour_row, neighbour_col));
            }
        }
    }

    pub fn kill_all(&mut self) {
        self.cells.iter_mut().for_each(|cell| cell.state = CellState::Dead);
    }

}
