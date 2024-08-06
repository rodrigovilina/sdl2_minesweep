#![deny(clippy::complexity)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::expect_used)]
#![deny(clippy::min_ident_chars)]
#![deny(clippy::panic)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::single_call_fn)]
// #![deny(clippy::restriction)]
// #![allow(clippy::question_mark_used)]
// #![allow(clippy::use_debug)]
// #![allow(clippy::print_stdout)]
// #![allow(clippy::print_stderr)]
// #![allow(clippy::wildcard_enum_match_arm)]
// #![allow(clippy::redundant_type_annotations)]
// #![allow(clippy::arithmetic_side_effects)]
// #![allow(clippy::missing_docs_in_private_items)]
// #![allow(clippy::allow_attributes_without_reason)]
// #![allow(clippy::std_instead_of_core)]
// #![allow(clippy::implicit_return)]
// #![allow(clippy::exit)]

extern crate sdl2;

mod board;
mod cell;
mod config;
mod coordinate;
mod front;

use {
  board::Board,
  config::bombs,
  front::Frontend,
  sdl2::{render::TextureCreator, video::WindowContext},
  std::{ops::ControlFlow, thread::sleep, time::Duration},
};

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), String> {
  let mut board = Board::new(bombs())?;
  let mut front: Frontend = Frontend::new()?;
  let texture_creator: TextureCreator<WindowContext> = front.canvas.texture_creator();

  front.initial_present();

  'running: loop {
    front.clear_click();

    let cont: ControlFlow<()> = front.process_events();
    match cont {
      ControlFlow::Continue(()) => {},
      ControlFlow::Break(()) => break 'running,
    }

    front.clear_screen();
    front.reveal_cell(&mut board);
    front.draw_board(&board, &texture_creator)?;
    println!("{:?}", board.progress());

    sleep(Duration::from_millis(100));
  }

  Ok(())
}
