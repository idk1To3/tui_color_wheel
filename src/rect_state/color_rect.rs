use ratatui::widgets::{Block, StatefulWidget, Widget};

use crate::RectState;

/// Defines a color wheel StatefulWidget in the shape of a rectangle, using a RectState as its state (just like
/// ColorEllipse). This can optionally be surrounded by a Block.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ColorRect<'a> {
    block: Option<Block<'a>>
}

impl<'a> ColorRect<'a> {
    pub fn new() -> ColorRect<'a> {
        ColorRect { block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> ColorRect<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for ColorRect<'a> {
    type State = RectState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
        where Self: Sized 
    {
        if let Some(block) = self.block {
            state.rect = block.inner(area);
            block.render(area, buf);
        }
        else {
            state.rect = area;
        }

        let sx = state.rect.x;
        let ex = state.rect.x + state.rect.width;
        let sy = state.rect.y;
        let ey = state.rect.y + state.rect.height;

        for x in sx..ex {
            for y in sy..ey {
                buf[(x, y)].set_char('█').set_fg(state.color(x,y));
            }
        }
    }
}
