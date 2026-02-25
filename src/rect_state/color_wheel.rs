use ratatui::widgets::{Block, StatefulWidget, Widget};

use crate::RectState;

/// Defines a color wheel StatefulWidget in the shape of an ellipse, using a RectState as its state (just like
/// ColorRect). This can optionally be surrounded by a Block.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ColorWheel<'a> {
    block: Option<Block<'a>>
}

impl<'a> ColorWheel<'a> {
    pub fn new() -> ColorWheel<'a> {
        ColorWheel { block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> ColorWheel<'a> {
        self.block = Some(block);
        self
    }

    fn draw(&self, buf: &mut ratatui::prelude::Buffer, state: &RectState) {
        let mut drawline = |sx: i32, ex: i32, y: i32| {
            for x in sx..=ex {
                let x = x as u16;
                let y = y as u16;
                buf[(x, y)].set_char('█').set_fg(state.color(x,y));
            }
        };
        let a = state.rect.width as i32 / 2;
        let b = state.rect.height as i32 / 2;

        let cx = state.rect.x as i32 + a;
        let cy = state.rect.y as i32 + b;

        let px = if state.rect.width  % 2 == 0 {1} else {0};
        let py = if state.rect.height % 2 == 0 {1} else {0};

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

impl<'a> StatefulWidget for ColorWheel<'a> {
    type State = RectState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
        where Self: Sized 
    {
        if let Some(block) = &self.block {
            for x in area.left() .. area.right() {
                for y in area.top() .. area.bottom() {
                    buf[(x,y)].set_char(' ');
                }
            }
            state.rect = block.inner(area);
            block.render(area, buf);
        }
        else {
            state.rect = area;
        }

        self.draw(buf, state);
    }
}
