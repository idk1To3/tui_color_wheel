use ratatui::{style::Style, widgets::StatefulWidget};

use crate::RectState;

/// Defines a StatefulWidget in the shape of a rectangle, using a RectState as its state (just like
/// ColorEllipse)
pub struct ColorRect;

impl StatefulWidget for ColorRect {
    type State = RectState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
        where Self: Sized 
    {
        if !state.rect_locked { 
            state.update_to_rect(area);
        }

        let sx = state.x;
        let ex = state.x + state.width;
        let sy = state.y;
        let ey = state.y + state.height;

        for x in sx..ex {
            for y in sy..ey {
                buf.set_stringn(x, y, "█", 1,
                    Style::default().fg(state.color(x, y))
                    );
            }
        }
    }
}
