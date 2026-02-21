//! # TUI color picker
//!
//! This is a crate designed for displaying color wheels for Ratatui applications,
//! from which the user could then select colors.
//!
//! This library defines three widgets (or more specifically, `StatefulWidget`s): 
//! ColorWheel, ColorRect and ColorEllipse. The first one is defined by a CircleState,
//! and the latter two are defined by a RectState, which is what you pass as the state
//! argument for render_stateful_widget. The widgets themselves are empty structs.
//! 
//! ## Examples
//!
//! Drawing a rectangle and a color wheel:
//! ```
//! use std::io;
//! use ratatui::layout::Rect;
//! use tui_color_picker::{CircleState, ColorRect, ColorWheel, RectState};
//!
//! fn main() -> io::Result<()> {
//!     let mut rect = RectState::new_with_white(1.0);
//! 
//!     let mut circle = CircleState::new(0.9, 0.9);
//! 
//!     ratatui::run(|terminal| -> io::Result<()> {
//!         loop {
//!             terminal.draw(|f| {
//!                 f.render_stateful_widget(ColorRect, Rect::new(10, 5, 20, 10), &mut rect);
//!                 f.render_stateful_widget(ColorWheel, Rect::new(35, 5, 12, 12), &mut circle);
//!             })?;
//!         }
//!     })?;
//! 
//!     Ok(())
//! }
//! ```
//!
//! Changing the color of a widget based on the input on a ColorEllipse:
//!
//! ```
//! use std::io::{self, stdout};
//! 
//! use ratatui::{DefaultTerminal, Frame, crossterm::execute, layout::Rect, style::{Color, Style}, text::Text};
//! use ratatui::crossterm::event::{self, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
//! 
//! struct App {
//!     color: Color,
//!     color_wheel: tui_color_picker::RectState,
//!     quit: bool,
//! }
//! 
//! impl App {
//!     fn new() -> App {
//!         App { 
//!             color: Color::default(), 
//!             color_wheel: tui_color_picker::RectState::new_with_white(1.0),
//!             quit: false,
//!         }
//!     }
//! 
//!     fn render(&mut self, frame: &mut Frame) {
//!         frame.render_widget(
//!             Text::from("Lorem ipsum").style(Style::default().fg(self.color)), 
//!             frame.area()
//!         );
//! 
//!         frame.render_stateful_widget(
//!             tui_color_picker::ColorEllipse, 
//!             Rect::new(10,10,20,10), 
//!             &mut self.color_wheel
//!         );
//!     }
//! 
//!     fn handle_events(&mut self) -> io::Result<()> {
//!         match event::read()? {
//!             event::Event::Key(KeyEvent{code: KeyCode::Char('q'), ..}) => {
//!                 self.quit = true;
//!             }
//! 
//!             event::Event::Mouse(MouseEvent{kind: MouseEventKind::Down(MouseButton::Left), column, row, ..}) => {
//!                 self.color = self.color_wheel.color(column, row);
//!             }
//!             _ => {}
//!         }
//! 
//!         Ok(())
//!     }
//! 
//!     fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
//!         while !self.quit {
//!             terminal.draw(|frame| self.render(frame))?;
//! 
//!             self.handle_events()?;
//!         }
//! 
//!         Ok(())
//!     }
//! }
//! 
//! fn main() -> io::Result<()> {
//!     let terminal = ratatui::init();
//! 
//!     execute!(stdout(), event::EnableMouseCapture)?;
//! 
//!     App::new().run(terminal)?;
//! 
//!     execute!(stdout(), event::DisableMouseCapture)?;
//! 
//!     ratatui::restore();
//! 
//!     Ok(())
//! }
//! ```

pub mod circle_state;
pub mod rect_state;

pub use circle_state::CircleState;
pub use rect_state::RectState;

pub use circle_state::color_wheel::ColorWheel;
pub use rect_state::color_rect::ColorRect;
pub use rect_state::color_ellipse::ColorEllipse;

/// h: (0, 360], s: \[0, 1\], v: \[0, 1\]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h = if h == 360.0 {0.0} else {h};

    let c = v * s; 
    let hp = h/60.0;
    let x = c * (1.0 - (hp.rem_euclid(2.0) - 1.0).abs());

    let (r1, g1, b1) = match hp {
        0.0..1.0 => (c,x,0.0),
        1.0..2.0 => (x,c,0.0),
        2.0..3.0 => (0.0,c,x),
        3.0..4.0 => (0.0,x,c),
        4.0..5.0 => (x,0.0,c),
        5.0..6.0 => (c,0.0,x),
        _ => (0.0, 0.0, 0.0),
    };

    let m = v - c;

    (r1 + m, g1 + m, b1 + m)
}

fn rad_to_deg(rad: f32) -> f32 {
    const FACTOR: f32 = 360.0 / std::f32::consts::TAU;
    (rad + std::f32::consts::PI) * FACTOR
}

/// value returned: \[0, 1\]
fn blend_parameter(x: f32, y: f32, radius: f32) -> f32 {
    (f32::sqrt(x*x + y*y) / radius).min(1.0)
}
