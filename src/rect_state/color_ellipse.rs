use ratatui::{style::Style, widgets::StatefulWidget};

use crate::RectState;

/// Defines a StatefulWidget in the shape of an ellipse, using a RectState as its state (just like
/// ColorRect).
pub struct ColorEllipse;

impl ColorEllipse {
    fn draw(&self, buf: &mut ratatui::prelude::Buffer, rect: &RectState) {
        let mut drawline = |sx: i32, ex: i32, y: i32| {
            for x in sx..=ex {
                buf.set_stringn(x as u16, y as u16, "█", 1, 
                    Style::default().fg(rect.color(x as u16, y as u16)));
            }
        };
        let a = rect.width as i32 / 2;
        let b = rect.height as i32 / 2;

        let cx = rect.x as i32 + a;
        let cy = rect.y as i32 + b;

        let px = if rect.width  % 2 == 0 {1} else {0};
        let py = if rect.height % 2 == 0 {1} else {0};

        let a2 = a*a;
        let b2 = b*b;

        let mut dx = 0;
        let mut dy = b;

        let mut d1 = 4*b2 - 4*b*a2 + a2;

        while b2 * dx <= a2 * dy {
            if d1 < 0 {
                dx += 1;
                d1 += 8*dx*b2 + 4*b2;
            }
            else {
                drawline(cx - dx, cx + dx - px, cy + dy - py);
                if dy != 0 {
                drawline(cx - dx, cx + dx - px, cy - dy);
                }

                dx += 1;
                dy -= 1;
                d1 += 8*dx*b2 - 8*dy*a2 + 4*b2;
            }
        }

        let mut d2 = (2*dx + 1)*(2*dx + 1)*b2 + 4*(dy - 1)*(dy - 1)*a2 - 4*a2*b2;

        while dy > 0 {
            drawline(cx - dx, cx + dx - px, cy + dy - py);
            drawline(cx - dx, cx + dx - px, cy - dy);

            if d2 > 0 {
                dy -= 1;
                d2 -= 8*dy*a2 + 4*a2;
            }
            else {
                dy -= 1;
                dx += 1;
                d2 += 8*dx*b2 - 8*dy*a2 + 4*a2;
            }
        }

        drawline(cx - dx, cx + dx - px, cy - py);
    }
}

impl StatefulWidget for ColorEllipse {
    type State = RectState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
        where Self: Sized 
    {
        if !state.rect_locked {
            state.update_to_rect(area);
        }

        self.draw(buf, state);
    }
}
