use {
  crate::{
    cell::{Cell, Status},
    config::{board_height_usize, board_width_usize},
    coordinate::Coordinate,
  },
  rand::{prelude::ThreadRng, Rng},
  std::collections::HashSet,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Progress {
  Lost,
  Won,
  Ongoing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
  pub cells: Vec<Vec<Cell>>,
}

impl Board {
  #[expect(clippy::single_call_fn, reason = "Constructor")]
  fn empty() -> Self {
    let cell: Cell = Cell {
      status: Status::ClosedClear,
      neighboring_bombs: 0,
    };
    let cells: Vec<Vec<Cell>> = vec![vec![cell; board_width_usize()]; board_height_usize()];
    Self { cells }
  }

  #[expect(clippy::single_call_fn, reason = "Constructor")]
  pub fn new(bombs: usize) -> Result<Self, &'static str> {
    let cell_quantity: usize = board_width_usize() * board_height_usize();
    if bombs > cell_quantity {
      return Err("Cannot generate more unique numbers than the range allows");
    }

    let mut rng: ThreadRng = rand::thread_rng();
    let mut bomb_indices: HashSet<usize> = HashSet::new();

    while bomb_indices.len() < bombs {
      bomb_indices.insert(rng.gen_range(0..cell_quantity));
    }

    let mut board: Self = Self::empty();

    for index in bomb_indices {
      #[expect(clippy::cast_possible_wrap)]
      let x = (index % board_width_usize()) as isize;
      #[expect(clippy::cast_possible_wrap)]
      let y = (index / board_width_usize()) as isize;
      if let Some(coord) = Coordinate::new(x, y) {
        board.add_bomb(&coord);
      }
    }

    Ok(board)
  }

  fn add_bomb(&mut self, coordinate: &Coordinate) {
    self.cells[coordinate.y][coordinate.x].plant_bomb();
    coordinate.adjacents().into_iter().for_each(|cord| {
      self.cells[cord.y][cord.x].increase_neighboring_bombs();
    });
  }

  pub fn reveal(&mut self, coordinate: &Coordinate) {
    let cell: &mut Cell = &mut self.cells[coordinate.y][coordinate.x];
    match cell.status {
      Status::ClosedBomb => {
        cell.status = Status::OpenBomb;
      },
      Status::OpenClear | Status::OpenBomb => {},
      Status::ClosedClear => {
        cell.status = Status::OpenClear;

        if cell.neighboring_bombs == 0 {
          coordinate.adjacents().into_iter().for_each(|coord| {
            self.reveal(&coord);
          });
        }
      },
    }
  }

  pub fn progress(&self) -> Progress {
    let mut progress = Progress::Won;
    for row in &self.cells {
      for cell in row {
        match cell.status {
          Status::ClosedClear => progress = Progress::Ongoing,
          Status::ClosedBomb | Status::OpenClear => {},
          Status::OpenBomb => return Progress::Lost,
        }
      }
    }

    progress
  }
}
