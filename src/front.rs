use {
  crate::{
    board::Board,
    cell::{Cell, Status},
    config::{
      board_height_usize, board_width_usize, cell_size_i32, cell_size_plus_separation_i32,
      cell_size_u32, window_height_u32, window_width_u32,
    },
    coordinate::Coordinate,
  },
  sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator, TextureQuery},
    surface::Surface,
    ttf::{Font, Sdl2TtfContext},
    video::{Window, WindowContext},
    EventPump, Sdl, VideoSubsystem,
  },
  std::ops::ControlFlow,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Click {
  pub coord: Coordinate,
}

pub struct Frontend {
  pub canvas: Canvas<Window>,
  pub events: EventPump,
  pub font: Font<'static, 'static>,
  pub click: Option<Click>,
}

impl Frontend {
  #[expect(clippy::single_call_fn, reason = "Constructor")]
  pub fn new() -> Result<Self, String> {
    let context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = context.video()?;
    let ttf_context: &mut Sdl2TtfContext =
      Box::leak(Box::new(sdl2::ttf::init().map_err(|err| err.to_string())?));

    let window: Window = video_subsystem
      .window("Mouse", window_width_u32(), window_height_u32())
      .position_centered()
      .build()
      .map_err(|err| err.to_string())?;

    let canvas: Canvas<Window> = window
      .into_canvas()
      .present_vsync()
      .build()
      .map_err(|err| err.to_string())?;

    let events: EventPump = context.event_pump()?;
    let font_path: &str = "small_5x3.ttf";
    let font_size: u16 = 24;
    let font: Font = ttf_context.load_font(font_path, font_size)?;

    Ok(Self {
      canvas,
      events,
      font,
      click: None,
    })
  }

  pub fn initial_present(&mut self) {
    self.canvas.set_draw_color(Color::RGB(255, 0, 0));
    self.canvas.clear();
    self.canvas.present();
  }

  pub fn clear_click(&mut self) {
    self.click = None;
  }

  pub fn clear_screen(&mut self) {
    self.canvas.set_draw_color(Color::RGB(20, 20, 20));
    self.canvas.clear();
  }

  pub fn reveal_cell(&self, board: &mut Board) {
    if let Some(click) = &self.click {
      println!("{click:?}");
      board.reveal(&click.coord);
    }
  }

  pub fn process_events(&mut self) -> ControlFlow<(), ()> {
    let events: Vec<Event> = self.events.poll_iter().collect();
    for event in events {
      match event {
        Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        }
        | Event::Quit { .. } => return ControlFlow::Break(()),
        Event::MouseButtonDown {
          timestamp: _,
          window_id: _,
          which: _,
          mouse_btn: _,
          clicks: _,
          x,
          y,
        } => {
          let row = y / cell_size_plus_separation_i32();
          let column = x / cell_size_plus_separation_i32();
          let x_within_cell = x % cell_size_plus_separation_i32();
          let y_within_cell = y % cell_size_plus_separation_i32();

          if x_within_cell < cell_size_i32() && y_within_cell < cell_size_i32() {
            self.update_click(column, row);
          }
        },
        _ => {},
      }
    }
    ControlFlow::Continue(())
  }

  fn update_click(&mut self, column: i32, row: i32) {
    self.click = Some(Click {
      coord: Coordinate::new(column as isize, row as isize).unwrap(),
    });
  }

  pub fn draw_board(
    &mut self,
    board: &Board,
    texture_creator: &TextureCreator<WindowContext>,
  ) -> Result<(), String> {
    for row_index in 0..board_height_usize() {
      for col_index in 0..board_width_usize() {
        #[expect(clippy::cast_possible_wrap)]
        #[expect(clippy::cast_possible_truncation)]
        let x: i32 = cell_size_plus_separation_i32() * col_index as i32;
        #[expect(clippy::cast_possible_wrap)]
        #[expect(clippy::cast_possible_truncation)]
        let y: i32 = cell_size_plus_separation_i32() * row_index as i32;
        let cell: Cell = board.cells[row_index][col_index];
        match cell.status {
          Status::ClosedBomb => {
            self.canvas.set_draw_color(Color::RGB(40, 40, 20));
          },
          Status::OpenBomb => {
            self.canvas.set_draw_color(Color::RGB(200, 40, 20));
          },
          Status::ClosedClear => {
            self.canvas.set_draw_color(Color::RGB(100, 100, 80));
          },
          Status::OpenClear => {
            self.canvas.set_draw_color(Color::RGB(160, 160, 80));
          },
        }
        self
          .canvas
          .fill_rect(Rect::new(x, y, cell_size_u32(), cell_size_u32()))?;
        if cell.neighboring_bombs != 0 && cell.status == Status::OpenClear {
          let text: String = format!("{}", cell.neighboring_bombs);
          let surface: Surface = self
            .font
            .render(&text)
            .blended(Color::RGBA(255, 200, 0, 255))
            .map_err(|err| err.to_string())?;
          let texture: Texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|err| err.to_string())?;

          let TextureQuery { width, height, .. } = texture.query();

          let target: Rect = Rect::new(x + 12, y + 6, width, height);
          self.canvas.copy(&texture, None, Some(target))?;
        }
      }
    }
    self.canvas.present();
    Ok(())
  }
}
