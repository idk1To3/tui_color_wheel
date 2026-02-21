use ratatui::{layout::Rect, style::Color};

use crate::{blend_parameter, hsv_to_rgb, rad_to_deg};

pub mod color_rect;
pub mod color_ellipse;

/// Defines a rectangle shape with its top left corner at x and y and with its bottom right corner
/// at (x + width - 1, y + height - 1).
///
/// The value is the V in HSV. A value of 0.0 will make the whole thing black.
///
/// By default, x, y, width and height are updated to fit the rectangle provided by the last call
/// to render_stateful_widget. You can prevent this from happening by locking the rectangle using 
/// the rect_locked variable or with calls to set_coordinates or set_dimensions.
pub struct RectState {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    value: Option<f32>,
    saturation: Option<f32>,
    pub rect_locked: bool,
}

impl RectState {
    /// Creates a new Rect. This will have a constant value and saturation across each slice.
    /// value, saturation: \[0.0, 1.0\]
    pub fn new(value: f32, saturation: f32) -> RectState {
        RectState { 
            x: 0, 
            y: 0, 
            width: 0,
            height: 0,
            value: Some(value.clamp(0.0, 1.0)), 
            saturation: Some(saturation.clamp(0.0, 1.0)), 
            rect_locked: false 
        }
    }
    /// Creates a new Rect. This will have a gradient to white towards its center, each slice, decreasing 
    /// in saturation and with a constant value
    /// value: \[0.0, 1.0\]
    pub fn new_with_white(value: f32) -> RectState {
        RectState { 
            x: 0, 
            y: 0, 
            width: 0,
            height: 0,
            value: Some(value.clamp(0.0, 1.0)), 
            saturation: None, 
            rect_locked: false 
        }
    }
    /// Creates a new Rect. This will have a gradient to black towards its center, each slice, decreasing 
    /// in value and with a constant saturation
    /// saturation: \[0.0, 1.0\]
    pub fn new_with_black(saturation: f32) -> RectState {
        RectState { 
            x: 0, 
            y: 0, 
            width: 0,
            height: 0,
            value: None, 
            saturation: Some(saturation.clamp(0.0, 1.0)), 
            rect_locked: false 
        }
    }

    /// Returns the Rgb color at the point in the screen defined by the coordinates x and y (0 indexed).
    ///
    /// Note: x and y do not necessarily have to be inside the ellipse/rectangle (or even inside the screen). 
    /// If they are outside, the color returned will be whatever would be at that ellipse's angle. 
    pub fn color(&self, x: u16, y: u16) -> Color {
        let squish = self.width as f32 / self.height as f32;

        let cx = self.x + self.width / 2;
        let cy = self.y + self.height / 2;

        let dx = (x as i32 - cx as i32) as f32;
        let dy = (y as i32 - cy as i32) as f32 * squish;

        let s = self.saturation.unwrap_or(
            blend_parameter(dx, dy, self.width as f32 / 2.0)
        );
        let v = self.value.unwrap_or(
            blend_parameter(dx, dy, self.width as f32 / 2.0)
        );

        let theta = rad_to_deg(f32::atan2(dy, dx));

        let rgb = hsv_to_rgb(theta, s,v);

        let r = (rgb.0 * 255.0) as u8;
        let g = (rgb.1 * 255.0) as u8;
        let b = (rgb.2 * 255.0) as u8;

        Color::Rgb(r,g,b)
    }

    fn update_to_rect(&mut self, rect: Rect) {
        self.x = rect.x;
        self.y = rect.y;

        self.width = rect.width;
        self.height = rect.height;
    }

    /// Gets the width and the height of the rectangle (in cells)
    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Gets the saturation. If it's None, that means there's a gradient.
    pub fn get_saturation(&self) -> Option<f32> {
        self.saturation
    }

    /// Gets the value. If it's None, that means there's a gradient.
    pub fn get_value(&self) -> Option<f32> {
        self.value
    }

    /// Unlocks the RectState. Now it's coordinatates and dimensions will be changed by every call 
    /// to render_stateful_widget, designed to fit inside the Rect provided by the area parameter.
    /// (This is the default behavior)
    pub fn unlock(&mut self) {
        self.rect_locked = false;
    }

    /// Sets the coordinates (top left corner) of the rectangle. 
    /// x and y are 0 indexed
    ///
    /// This will lock the RectState (prevent it's coordinates and dimensions from being 
    /// changed by future calls to render_stateful_widget)
    pub fn set_coordinates(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    /// Sets the dimensions of the rectangle (in cells).
    ///
    /// This will lock the RectState (prevent it's coordinates and dimensions from being 
    /// changed by future calls to render_stateful_widget)
    pub fn set_dimensions(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Makes the shape have a constant saturation across each slice.
    pub fn set_to_constant_saturation(&mut self, saturation: f32) {
        self.saturation = Some(saturation);
    }

    /// Makes the shape have a constant value across each slice.
    pub fn set_to_constant_value(&mut self, value: f32) {
        self.value = Some(value);
    }

    /// Makes the saturation of each cell increase based on the distance from the center.
    /// Sets the shape to have a gradient to white towards its center.
    ///
    /// If both saturation and value are set to have gradients, the resulting shape will
    /// be black at the center and with grayish colors inbetween.
    pub fn set_to_saturation_gradient(&mut self) {
        self.saturation = None;
    }

    /// Makes the value of each cell increase based on the distance from the center.
    /// Sets the shape to have a gradient to black towards its center.
    ///
    /// If both saturation and value are set to have gradients, the resulting shape will
    /// be black at the center and with grayish colors inbetween.
    pub fn set_to_value_gradient(&mut self) {
        self.value = None;
    }
}
