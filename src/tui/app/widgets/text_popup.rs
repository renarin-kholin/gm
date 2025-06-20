use crossterm::event::KeyCode;
use ratatui::widgets::{Block, Widget};

use super::{popup::Popup, text_scroll::TextScroll};
use crate::tui::theme::Theme;
use crate::tui::{
    traits::{BorderedWidget, HandleResult},
    Event,
};

pub struct TextPopup {
    title: &'static str,
    text_scroll: TextScroll,
    theme: Theme,
}

impl TextPopup {
    pub fn new(title: &'static str, theme: Theme) -> Self {
        let text = String::new();
        Self {
            title,
            text_scroll: TextScroll::new(text),
            theme,
        }
    }

    pub fn is_shown(&self) -> bool {
        !self.text_scroll.text.is_empty()
    }

    pub fn clear(&mut self) {
        self.text_scroll.text.clear();
    }

    pub fn set_text(&mut self, text: String) {
        self.text_scroll.text = text;
        // self.text_scroll.scroll_offset = 0;
    }

    pub fn handle_event(
        &mut self,
        event: &Event,
        area: ratatui::prelude::Rect,
    ) -> crate::Result<HandleResult> {
        let mut result = HandleResult::default();

        result.merge(self.text_scroll.handle_event(event, area)?);

        #[allow(clippy::single_match)]
        match event {
            Event::Input(key) => match key.code {
                KeyCode::Esc => {
                    self.clear();
                }
                _ => {}
            },
            _ => {}
        }

        if self.is_shown() {
            result.esc_ignores = 1;
        }

        Ok(result)
    }
}

impl Widget for &TextPopup {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if self.text_scroll.text.is_empty() {
            return;
        }
        Popup.render(area, buf);

        let popup_inner_area = Popup::inner_area(area);

        let block = Block::bordered()
            .style(&self.theme)
            .title(self.title)
            .title_bottom("press ESC to dismiss");

        self.text_scroll
            .render_with_block(popup_inner_area, buf, block);
    }
}
