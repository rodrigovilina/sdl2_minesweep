#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
  ClosedBomb,
  ClosedClear,
  OpenClear,
  OpenBomb,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
  pub status: Status,
  pub neighboring_bombs: usize,
}

impl Cell {
  pub fn increase_neighboring_bombs(&mut self) {
    self.neighboring_bombs += 1;
  }

  pub fn plant_bomb(&mut self) {
    self.status = Status::ClosedBomb;
  }
}
