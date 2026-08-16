use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Span::from(" DLTUI VIEWER ").bold().underlined())
            .title_top(Line::from(self.render_auto_scroll_badge()).right_aligned())
            .title_bottom(
                Span::from(
                    " Open <o> | ECUs <e> | Filters <f> | Auto-Scroll <a> | Save <s> | Settings <c> | Quit <q> ",
                )
                .bold(),
            )
            .title_alignment(Alignment::Center);

        let text = format!("WELCOME TO DLTUI VIEWER");

        let paragraph = Paragraph::new(text).block(block).centered();

        paragraph.render(area, buf);
    }
}

impl App {
    fn render_auto_scroll_badge(&self) -> Span<'static> {
        if self.context.dlp.settings.other.auto_scroll {
            Span::from(" AUTO SCROLL [X] ")
                .add_modifier(Modifier::BOLD)
                .green()
        } else {
            Span::from(" AUTO SCROLL [ ] ")
        }
    }
}
