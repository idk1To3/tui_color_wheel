//! # TUI color picker
//!
//! This is a crate designed for displaying color wheels for Ratatui applications, from which the 
//! user could then select colors.
//!
//! This library defines two widgets (or more specifically, `StatefulWidget`s): ColorRect and 
//! ColorWheel. Both of these use RectState to store their state, such as information about where
//! they are, which is needed for getting input from them.
//! 
//! ## Examples
//!
//! Drawing a rectangle and a color wheel (the first one having a gradient and a border):
//! ```no_run
//! use std::io;
//! use ratatui::{layout::Rect, widgets::Block};
//! use tui_color_picker::{ColorRect, ColorWheel, RectState};
//!
//! fn main() -> io::Result<()> {
//!     let mut rect_gradient = RectState::new_with_white(1.0);
//! 
//!     let mut rect_no_gradient = RectState::new(0.9, 0.9);
//! 
//!     ratatui::run(|terminal| -> io::Result<()> {
//!         loop {
//!             terminal.draw(|f| {
//!                 f.render_stateful_widget(ColorRect::new().block(Block::bordered()), 
//!                                          Rect::new(10, 5, 20, 10), 
//!                                          &mut rect_gradient);
//!                 f.render_stateful_widget(ColorWheel::new(), 
//!                                          Rect::new(35, 5, 12, 12), 
//!                                          &mut rect_no_gradient);
//!             })?;
//!         }
//!     })?;
//! 
//!     Ok(())
//! }
//! ```
//!
//! Changing the color of a widget based on the input on a ColorWheel:
//!
//! ```no_run
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
//!             tui_color_picker::ColorWheel::new(), 
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
//!
//! Elaborate example that promps the user to pick a color for a paragraph or a title whenever he
//! right clicks on them. 
//!
//!```no_run
//! use std::io::{self, stdout};
//! 
//! use ratatui::{DefaultTerminal, Frame, layout::{Position, Rect, Size}, macros::{horizontal, vertical}, style::{Color, Style}, widgets::{Wrap, Block, Paragraph}};
//! use ratatui::crossterm::{event::{self, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind}, execute};
//! use tui_color_picker::{ColorWheel, RectState};
//! 
//! struct App {
//!     quit: bool,
//!     areas: Vec<Panel>,
//!     color_popup: Option<Popup>
//! }
//! 
//! #[derive(Default, Clone)]
//! struct Panel {
//!     rect: Rect,
//!     border_color: Color,
//!     text_color: Color,
//! }
//! 
//! struct Popup {
//!     rect: Rect,
//!     color_wheel: tui_color_picker::RectState,
//!     area_index: usize,
//!     on_border: bool,
//! }
//! 
//! impl Popup {
//!     const WIDTH: u16 = 30 + 4; // the border is 2 cells wide left and right, so we add 4
//!     const HEIGHT: u16 = 15 + 4;
//! 
//!     fn new(x: u16, y: u16, area_index: usize, on_border: bool, screen_size: Size) -> Popup {
//!         let popup_x = if x + Self::WIDTH > screen_size.width {x - Self::WIDTH + 1} else {x};
//!         let popup_y = if y + Self::HEIGHT > screen_size.height {y - Self::HEIGHT + 1} else {y};
//! 
//!         let color_wheel = RectState::new_with_white(1.0);
//! 
//!         let rect = Rect::new(popup_x, popup_y, Self::WIDTH, Self::HEIGHT);
//! 
//!         Popup { rect, color_wheel, area_index, on_border }
//!     }
//! }
//! 
//! impl App {
//!     const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
//! 
//!     fn new() -> App {
//!         App { 
//!             color_popup: None,
//!             quit: false,
//!             areas: vec![Panel::default(); 3],
//!         }
//!     }
//! 
//!     fn render(&mut self, frame: &mut Frame) {
//!         let rects = horizontal![*= 3, *= 2].split(frame.area());
//!         let right = rects[1];
//!         let rects = vertical![*= 4, *= 3].split(rects[0]);
//! 
//!         self.areas[0].rect = rects[0];
//!         self.areas[1].rect = rects[1];
//!         self.areas[2].rect = right;
//! 
//!         for (i, area) in self.areas.iter().enumerate() {
//!             let title_style = Style::default().bold().fg(area.border_color);
//!             let text_style = Style::default().fg(area.text_color);
//! 
//!             let block = Block::bordered().title(format!("Paragraph {i}")).style(title_style);
//!             let paragraph = Paragraph::new(Self::LOREM_IPSUM)
//!                 .style(text_style)
//!                 .wrap(Wrap::default())
//!                 .block(block);
//! 
//!             frame.render_widget(paragraph, area.rect);
//!         }
//! 
//!         if let Some(color_popup) = &mut self.color_popup {
//!             let block = Block::bordered()
//!                 .title("Pick a color: ")
//!                 .style(Style::default().fg(Color::White))
//!                 .padding(ratatui::widgets::Padding::symmetric(1, 1));
//! 
//!             frame.render_stateful_widget(ColorWheel::new().block(block), 
//!                 color_popup.rect,
//!                 &mut color_popup.color_wheel);
//!         }
//!     }
//! 
//! 
//!     fn handle_events(&mut self, terminal: &DefaultTerminal) -> io::Result<()> {
//!         match event::read()? {
//!             event::Event::Key(KeyEvent{code: KeyCode::Char('q'), ..}) => {
//!                 self.quit = true;
//!             }
//! 
//!             event::Event::Mouse(MouseEvent{kind: MouseEventKind::Down(MouseButton::Left), column: x, row: y, ..}) => {
//!                 if let Some(popup) = &self.color_popup {
//!                     if popup.color_wheel.rect().contains(Position{x,y}) {
//!                         let color = popup.color_wheel.color(x, y);
//!                         if popup.on_border {
//!                             self.areas[popup.area_index].border_color = color;
//!                         }
//!                         else {
//!                             self.areas[popup.area_index].text_color = color;
//!                         }
//!                     }
//! 
//!                     self.color_popup = None;
//!                 }
//!             }
//! 
//!             event::Event::Mouse(MouseEvent{kind: MouseEventKind::Down(MouseButton::Right), column: x, row: y, ..}) => {
//!                 for (i, area) in self.areas.iter_mut().enumerate() {
//!                     if area.rect.contains(Position::new(x, y)) {
//!                         let on_border = x == area.rect.x || x == area.rect.x + area.rect.width - 1 ||
//!                                               y == area.rect.y || y == area.rect.y + area.rect.height - 1;
//! 
//!                         self.color_popup = Some(Popup::new(x, y, i, on_border, terminal.size()?));
//! 
//!                         break;
//!                     }
//!                 }
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
//!             self.handle_events(&terminal)?;
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
//!```

pub mod rect_state;

pub use rect_state::RectState;

pub use rect_state::color_rect::ColorRect;
pub use rect_state::color_wheel::ColorWheel;

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
        _ => { return (0.0, 0.0, 0.0); }
    };

    let m = v - c;

    (r1 + m, g1 + m, b1 + m)
}

fn rad_to_deg(rad: f32) -> f32 {
    const FACTOR: f32 = 360.0 / std::f32::consts::TAU;
    rad * FACTOR
}

fn blend_parameter(x: f32, y: f32, radius: f32) -> f32 {
    (f32::sqrt(x*x + y*y) / radius).min(1.0)
}
