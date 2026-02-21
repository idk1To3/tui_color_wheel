use ratatui::{layout::Rect, style::Color};
use crate::{blend_parameter, hsv_to_rgb, rad_to_deg};

pub mod color_wheel;

/// Defines a circle shape with its center at x and y. The
/// diameter is equal to 2*radius + 1. Thus the rectangle the circle is inside has the top left
/// corner at (x-radius, y-radius) and the bottom right corner at (x+radius, y+radius).
///
/// The value is the V in HSV. A value of 0.0 will make the whole color wheel black.
///
/// By default, x, y and radius are updated to fit the rectangle provided by the last call
/// to render_stateful_widget. You can prevent this from happening by locking the circle using the
/// circle_locked variable or with calls to set_center or set_radius.
///
/// Note: fonts typically have characters that are rectangles, not squares. Thus a color wheel in
/// the shape of a circle will appear like an oval for most users. Consider using a RectState instead,
/// from which you can then create a ColorEllipse widget or a ColorRect widget
pub struct CircleState {
    x: u16,
    y: u16,
    radius: u16,
    value: Option<f32>,
    saturation: Option<f32>,
    pub circle_locked: bool,
}

impl CircleState {
    /// Creates a new CircleState. This will have a constant value and saturation across each slice.
    /// value, saturation: \[0.0, 1.0\]
    pub fn new(value: f32, saturation: f32) -> CircleState {
        CircleState { 
            x: 0, 
            y: 0, 
            radius: 0, 
            value: Some(value.clamp(0.0, 1.0)), 
            saturation: Some(saturation.clamp(0.0, 1.0)), 
            circle_locked: false 
        }
    }
    /// Creates a new CircleState. This will have a gradient to white towards its center, each slice 
    /// decreasing in saturation and with a constant value
    /// value: \[0.0, 1.0\]
    pub fn new_with_white(value: f32) -> CircleState {
        CircleState { 
            x: 0, 
            y: 0, 
            radius: 0, 
            value: Some(value.clamp(0.0, 1.0)), 
            saturation: None, 
            circle_locked: false 
        }
    }
    /// Creates a new CircleState. This will have a gradient to black towards its center, each slice 
    /// decreasing in value and with a constant saturation
    /// saturation: \[0.0, 1.0\]
    pub fn new_with_black(saturation: f32) -> CircleState {
        CircleState { 
            x: 0, 
            y: 0, 
            radius: 0, 
            value: None, 
            saturation: Some(saturation.clamp(0.0, 1.0)), 
            circle_locked: false 
        }
    }

    /// Returns the Rgb color at the point in the screen defined by the coordinates x and y (0 indexed).
    ///
    /// Note: x and y do not necessarily have to be inside the circle (or even inside the screen). 
    /// If they are outside the circle, the color returned will be whatever would be at that angle. 
    pub fn color(&self, x: u16, y: u16) -> Color {
        let dx = (x as i32 - self.x as i32) as f32;
        let dy = (y as i32 - self.y as i32) as f32;

        let s = self.saturation.unwrap_or(
            blend_parameter(dx, dy, self.radius as f32)
        );
        let v = self.value.unwrap_or(
            blend_parameter(dx, dy, self.radius as f32)
        );

        let theta = rad_to_deg(f32::atan2(dy, dx));

        let rgb = hsv_to_rgb(theta, s,v);

        let r = (rgb.0 * 255.0) as u8;
        let g = (rgb.1 * 255.0) as u8;
        let b = (rgb.2 * 255.0) as u8;

        Color::Rgb(r,g,b)
    }

    fn update_to_rect(&mut self, rect: Rect) {
        self.radius = (rect.width.min(rect.height) - 1) / 2;
        self.x = rect.x + self.radius;
        self.y = rect.y + self.radius;
    }

    /// Gets the radius of the circle (in cells)
    pub fn get_radius(&self) -> u16 {
        self.radius
    }

    /// Gets the saturation. If it's None, that means there's a gradient.
    pub fn get_saturation(&self) -> Option<f32> {
        self.saturation
    }

    /// Gets the value. If it's None, that means there's a gradient.
    pub fn get_value(&self) -> Option<f32> {
        self.value
    }

    /// Unlocks the CircleState. Now it's coordinatates and radius will be changed by every call to
    /// render_stateful_widget, designed to fit inside the Rect provided by the area parameter.
    /// (This is the default behavior)
    pub fn unlock(&mut self) {
        self.circle_locked = false;
    }

    /// Sets the center of the circle. 
    /// x and y are 0 indexed
    ///
    /// This will lock the CircleState (prevent it's coordinates and radius from being 
    /// changed by future calls to render_stateful_widget)
    pub fn set_center(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    /// Sets the radius of the circle (in cells).
    /// The radius will be clamped to be equal to or greater than 1.
    ///
    /// This will lock the CircleState (prevent it's coordinates and radius from being 
    /// changed by future calls to render_stateful_widget)
    pub fn set_radius(&mut self, radius: u16) {
        self.radius = radius.min(1);
        self.circle_locked = true;
    }

    /// Makes the color wheel have a constant saturation across each slice.
    pub fn set_to_constant_saturation(&mut self, saturation: f32) {
        self.saturation = Some(saturation);
    }

    /// Makes the color wheel have a constant value across each slice.
    pub fn set_to_constant_value(&mut self, value: f32) {
        self.value = Some(value);
    }

    /// Makes the saturation of each cell increase based on the distance from the center.
    /// Sets the circle to have a gradient to white towards its center.
    ///
    /// If both saturation and value are set to have gradients, the resulting color wheel will
    /// be black at the center and with grayish colors inbetween.
    pub fn set_to_saturation_gradient(&mut self) {
        self.saturation = None;
    }

    /// Makes the value of each cell increase based on the distance from the center.
    /// Sets the circle to have a gradient to black towards its center.
    ///
    /// If both saturation and value are set to have gradients, the resulting color wheel will
    /// be black at the center and with grayish colors inbetween.
    pub fn set_to_value_gradient(&mut self) {
        self.value = None;
    }
}

