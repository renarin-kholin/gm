use crate::tui::theme::Theme;
use crate::tui::traits::BorderedWidget;
use ratatui::{layout::Rect, style::Style, text::Line, widgets::Block};

pub struct Button {
    pub focus: bool,
    pub label: &'static str,
}

impl Button {
    pub fn render(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        theme: &Theme,
    ) where
        Self: Sized,
    {
        let button_area = Rect {
            width: (self.label.len() + 2) as u16,
            height: 3,
            x: area.x,
            y: area.y,
        };

        Line::from(self.label).render_with_block(
            button_area,
            buf,
            Block::bordered()
                .border_type(theme.into())
                .style(if self.focus {
                    theme.button_focused()
                } else {
                    Style::default()
                }),
        );
    }
}
