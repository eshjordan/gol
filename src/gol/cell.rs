#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn living(&self) -> bool {
        self.state == CellState::Alive
    }
}
