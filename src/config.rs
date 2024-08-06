#![allow(clippy::single_call_fn)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]

use std::{env, process, sync::LazyLock};

const CELL_SIZE: usize = 32;
const CELL_SEPARATION: usize = 4;

static ARGS: LazyLock<Vec<String>> = LazyLock::new(|| {
  let args: Vec<_> = env::args().collect();

  if args.len() < 4 {
    #[expect(clippy::unwrap_used)]
    let program_name = args.first().unwrap();
    eprintln!("Usage: {program_name} <width> <height> <bombs>");
    process::exit(1);
  }

  args
});

static BOARD_WIDTH: LazyLock<usize> = LazyLock::new(|| {
  #[expect(clippy::expect_used)]
  #[expect(clippy::unwrap_used)]
  args()
    .get(1)
    .unwrap()
    .parse::<usize>()
    .expect("Please provide a valid integer for width")
});

static BOARD_HEIGHT: LazyLock<usize> = LazyLock::new(|| {
  #[expect(clippy::expect_used)]
  #[expect(clippy::unwrap_used)]
  args()
    .get(2)
    .unwrap()
    .parse::<usize>()
    .expect("Please provide a valid integer for height")
});

pub static BOMBS: LazyLock<usize> = LazyLock::new(|| {
  #[expect(clippy::expect_used)]
  #[expect(clippy::unwrap_used)]
  args()
    .get(3)
    .unwrap()
    .parse::<usize>()
    .expect("Please provide a valid integer for the number of bombs")
});

pub fn args() -> Vec<String> {
  ARGS.clone()
}

pub fn bombs() -> usize {
  *BOMBS
}

static WINDOW_WIDTH: LazyLock<usize> =
  LazyLock::new(|| ((CELL_SIZE + CELL_SEPARATION) * *BOARD_WIDTH) - CELL_SEPARATION);

static WINDOW_HEIGHT: LazyLock<usize> =
  LazyLock::new(|| ((CELL_SIZE + CELL_SEPARATION) * *BOARD_HEIGHT) - CELL_SEPARATION);

pub const fn cell_size_usize() -> usize {
  CELL_SIZE
}

pub const fn cell_size_u32() -> u32 {
  cell_size_usize() as u32
}

pub const fn cell_size_i32() -> i32 {
  cell_size_usize() as i32
}

pub const fn cell_size_plus_separation_usize() -> usize {
  CELL_SIZE + CELL_SEPARATION
}

pub const fn cell_size_plus_separation_i32() -> i32 {
  cell_size_plus_separation_usize() as i32
}

pub fn board_width_usize() -> usize {
  *BOARD_WIDTH
}

pub fn board_height_usize() -> usize {
  *BOARD_HEIGHT
}

pub fn board_width_isize() -> isize {
  board_width_usize() as isize
}

pub fn board_height_isize() -> isize {
  board_height_usize() as isize
}

pub fn window_width_u32() -> u32 {
  *WINDOW_WIDTH as u32
}

pub fn window_height_u32() -> u32 {
  *WINDOW_HEIGHT as u32
}
