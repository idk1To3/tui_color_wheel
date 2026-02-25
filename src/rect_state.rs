use ratatui::{layout::Rect, style::Color};

use crate::{blend_parameter, hsv_to_rgb, rad_to_deg};

pub mod color_rect;
pub mod color_wheel;

/// Defines a rectangle shape and a bunch of other info needed by color wheels. 
/// This is the underlying data that both widgets use; whenever you render a color wheel
/// or a color rect, you need to provide a mutable reference to a RectState.
/// This will cause the struct's internal area to be changed to fit the area parameter 
/// from render_stateful_widget (subtracting the area by any borders or padding).
///
///```ignore
/// use tui_color_picker::{RectState, ColorWheel};
/// use ratatui::{widgets::Block, layout::Rect};
///
/// let state = RectState::new(1.0, 1.0);
///
/// // inside the terminal.draw() closure:
/// frame.render_stateful_widget(
///     ColorWheel::new().block(Block::bordered()),
///     Rect::new(0, 0, 30, 30),
///     &mut state);
///
/// assert_eq!(state.rect(), Rect{x: 1, y: 1, width: 28, height: 28});
///```
///
/// You can use this struct for input by calling color(x,y). For example, if a user left clicks
/// somewhere on the screen while a color wheel is visible, you can call color(col,row) on this 
/// struct to get whatever color is at that cell. 
///
/// Example using crossterm:
///```no_run
/// use ratatui::crossterm::event::{self, MouseButton, MouseEvent, MouseEventKind};
///
/// # fn handle_event() -> std::io::Result<()> {
/// let state = tui_color_picker::RectState::new(1.0, 1.0);
///
/// match event::read()? {
///     event::Event::Mouse(MouseEvent{kind: MouseEventKind::Down(MouseButton::Left), column: x, row: y, ..}) => {
///         println!("You selected the color: {}", state.color(x, y));
///     }
///     _ => {}
/// }
/// # Ok(())
/// # }
///
///```
/// As previously stated, the area for the color wheel is based on the area parameter provided by
/// the last call to render_stateful_widget. But what happens if you have never called the render
/// function? In that case, assuming you hadn't manually set the Rect using set_internal_rect(),
/// the color method would always return black.
///
/// Other than the rect, this struct also contains information about the colors (their saturation
/// and value and whether or not they have a gradient) as well as information about their
/// positioning (an offset and a direction).
#[derive(Debug, Clone, PartialEq)]
pub struct RectState {
    pub(crate) rect: Rect,
    value: Option<f32>,
    saturation: Option<f32>,
    /// This defines how many degrees is the hue offset along the clockwise direction.
    /// Does not have to be in the [0, 360) range.
    /// offset = 0 (or any other multiple of 360) means that red is at the right.
    pub offset: f32,
    direction: f32,
}

impl Default for RectState {
    fn default() -> Self {
        RectState { rect: Rect::default(), value: None, saturation: None, offset: 0.0, direction: Self::CLOCKWISE }
    }
}

impl RectState {
    const CLOCKWISE: f32 = -1.0;
    const ANTICLOCKWISE: f32 = 1.0;

    /// Creates a new Rect. This will have a constant value and saturation across each slice.
    /// saturation, value: \[0.0, 1.0\]
    pub fn new(saturation: f32, value: f32) -> RectState {
        RectState { 
            rect: Rect::default(),
            value: Some(value.clamp(0.0, 1.0)), 
            saturation: Some(saturation.clamp(0.0, 1.0)), 
            offset: 0.0,
            direction: Self::CLOCKWISE,
        }
    }
    /// Creates a new Rect. This will have a gradient to white towards its center, each slice decreasing 
    /// in saturation and with a constant value
    /// value: \[0.0, 1.0\]
    pub fn new_with_white(value: f32) -> RectState {
        RectState { 
            rect: Rect::default(),
            value: Some(value.clamp(0.0, 1.0)), 
            saturation: None, 
            offset: 0.0,
            direction: Self::CLOCKWISE,
        }
    }
    /// Creates a new Rect. This will have a gradient to black towards its center, each slice decreasing 
    /// in value and with a constant saturation
    /// saturation: \[0.0, 1.0\]
    pub fn new_with_black(saturation: f32) -> RectState {
        RectState { 
            rect: Rect::default(),
            value: None, 
            saturation: Some(saturation.clamp(0.0, 1.0)), 
            offset: 0.0,
            direction: Self::CLOCKWISE,
        }
    }

    /// Returns the Rgb color at the point in the screen defined by the coordinates x and y (0 indexed).
    /// If the rect isn't initialized with a meaningful value (either automatically by passing it to
    /// render_stateful_widget, or manually with a call to set_internal_rect), then this will
    /// return Color::Black;
    ///
    /// Note: x and y do not necessarily have to be inside the ellipse/rectangle (or even inside the screen). 
    /// If they are outside, the color returned will be whatever would be at that ellipse's angle. 
    pub fn color(&self, x: u16, y: u16) -> Color {
        let width = self.rect.width;
        let height = self.rect.height;

        let squish = width as f32 / height as f32;

        let cx = self.rect.x + width / 2;
        let cy = self.rect.y + height / 2;

        let dx = (x as i32 - cx as i32) as f32;
        let dy = (y as i32 - cy as i32) as f32 * squish;

        let s = self.saturation.unwrap_or(
            blend_parameter(dx, dy, width as f32 / 2.0)
        );
        let v = self.value.unwrap_or(
            blend_parameter(dx, dy, width as f32 / 2.0)
        );

        let mut theta = rad_to_deg(f32::atan2(dy, dx));

        theta = ((theta + self.offset) * self.direction).rem_euclid(360.0);

        let rgb = hsv_to_rgb(theta, s,v);

        let r = (rgb.0 * 255.0) as u8;
        let g = (rgb.1 * 255.0) as u8;
        let b = (rgb.2 * 255.0) as u8;

        Color::Rgb(r,g,b)
    }

    /// The rectangle that contains the color wheel itself (and not any borders)
    pub fn rect(&self) -> Rect {
        self.rect
    }

    /// Sets the internal rect that contains the color wheel, without any borders or padding.
    /// Note that this value will get overwritten the moment you render a color shape with this
    /// RectState.
    ///
    /// This method is useful if, for some weird reason, you wish to call color(x,y) without drawing 
    /// a widget with this. Normally the internal area is based on the last call to render, but if
    /// you don't render a widget but still want the functionality, you can manually set the area
    /// to where the color wheel *would* be.
    ///
    /// This can also be used to initialize the internal rect to a meaningful value if your TUI 
    /// loop has input logic before drawing logic, and you want to handle the edge case where a 
    /// call to color() is made before a widget has ever been drawn with this state. color() 
    /// always returns the color black if the rect is "uninitialized"
    pub fn set_internal_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    /// Is true if the hue goes in RGB order along the clockwise direction
    pub fn is_clockwise(&self) -> bool {
        self.direction == Self::CLOCKWISE
    }

    /// Set it to true if you want the hue to go in RGB order along the clockwise direction (the default)
    pub fn set_clockwise_direction(&mut self, clockwise: bool) {
        self.direction = if clockwise { Self::CLOCKWISE } else { Self::ANTICLOCKWISE };
    }


    /// Gets the saturation.
    /// If it's None, that means there's a saturation gradient across each slice.
    pub fn get_saturation(&self) -> Option<f32> {
        self.saturation
    }

    /// Gets the value (i.e. the V in HSV; a value of 0 corresponds to black). 
    /// If it's None, that means there's a value gradient across each slice.
    pub fn get_value(&self) -> Option<f32> {
        self.value
    }

    /// Makes the color wheel have a constant saturation across each slice.
    pub fn set_to_constant_saturation(&mut self, saturation: f32) {
        self.saturation = Some(saturation.clamp(0.0, 1.0));
    }

    /// Makes the color wheel have a constant value across each slice.
    pub fn set_to_constant_value(&mut self, value: f32) {
        self.value = Some(value.clamp(0.0, 1.0));
    }

    /// Makes the saturation of each cell increase based on the distance from the center.
    /// Sets the color wheel to have a gradient to white towards its center.
    ///
    /// If both saturation and value are set to have gradients, the resulting color wheel 
    /// will be black at the center and with grayish colors inbetween.
    pub fn set_to_saturation_gradient(&mut self) {
        self.saturation = None;
    }

    /// Makes the value of each cell increase based on the distance from the center.
    /// Sets the color wheel to have a gradient to black towards its center.
    ///
    /// If both saturation and value are set to have gradients, the resulting color wheel 
    /// will be black at the center and with grayish colors inbetween.
    pub fn set_to_value_gradient(&mut self) {
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use crate::RectState;

    #[test]
    fn unitialized_dimensions() {
        for s in [0.0, 1.0] {
            for v in [0.0, 1.0] {
                for x in [0, 3] {
                    for y in [0, 2] {
                        assert_eq!(RectState::new(s,v).color(x,y), Color::Rgb(0,0,0));
                        assert_eq!(RectState::new_with_black(s).color(x,y), Color::Rgb(0,0,0));
                        assert_eq!(RectState::new_with_white(v).color(x,y), Color::Rgb(0,0,0));
                    }
                }
            }
        }
    }
}
