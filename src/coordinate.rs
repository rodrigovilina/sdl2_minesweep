use crate::config::{board_height_isize, board_width_isize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinate {
  pub x: usize,
  pub y: usize,
}

impl Coordinate {
  const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
  ];

  pub fn new(x: isize, y: isize) -> Option<Self> {
    match (usize::try_from(x), usize::try_from(y)) {
      (Ok(xu), Ok(yu)) if x < board_width_isize() && y < board_height_isize() => {
        Some(Self { x: xu, y: yu })
      },
      _ => None,
    }
  }

  pub fn adjacents(&self) -> Vec<Self> {
    Self::DELTAS
      .into_iter()
      .filter_map(|(dx, dy)| {
        #[expect(clippy::cast_possible_wrap)]
        let x = self.x as isize + dx;
        #[expect(clippy::cast_possible_wrap)]
        let y = self.y as isize + dy;
        Self::new(x, y)
      })
      .collect()
  }
}
