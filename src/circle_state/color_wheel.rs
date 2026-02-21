use ratatui::{style::Style, widgets::StatefulWidget};

use crate::CircleState;

/// Defines a StatefulWidget in the shape of a circle, using a CircleState as its state.
/// This will be as wide in columns as it is tall in rows.
///
/// Note: fonts typically have characters that are rectangles, not squares. Thus a color wheel in
/// the shape of a circle will appear like an oval for most users. Consider using a ColorEllipse or
/// a ColorRect instead, for which you'd need a RectState.
pub struct ColorWheel;

impl ColorWheel {
    fn draw(&self, buf: &mut ratatui::prelude::Buffer, circle: &CircleState) {
        let mut drawline = |sx: i16, ex: i16, y: i16| {
            for x in sx..=ex {
                buf.set_stringn(x as u16, y as u16, "█", 1, 
                    Style::default().fg(circle.color(x as u16, y as u16)));
            }
        };

        let radius = circle.get_radius() as i16;
        let center_x = circle.x as i16;
        let center_y = circle.y as i16;

        let mut x0 = 0;
        let mut y0 = radius;
        let mut d = 3 - 2 * radius;

        while y0 >= x0 {
            drawline(center_x - y0, center_x + y0, center_y - x0);
            if x0 > 0 {
                drawline(center_x - y0, center_x + y0, center_y + x0);
            }
            if d < 0 {
                d += 4 * x0 + 6;
                x0 += 1;
            }
            else {
                if x0 != y0 {
                    drawline(center_x - x0, center_x + x0, center_y - y0);
                    drawline(center_x - x0, center_x + x0, center_y + y0);
                }
                d += 4 * (x0 - y0) + 10;
                x0 += 1;
                y0 -= 1;
            }
        }
    }
}

impl StatefulWidget for ColorWheel {
    type State = CircleState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) 
        where Self: Sized 
    {
        if !state.circle_locked {
            state.update_to_rect(area);
        }

        self.draw(buf, state);
    }
}
