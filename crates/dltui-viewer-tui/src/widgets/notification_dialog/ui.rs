use hyphenation::Load;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Styled, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget, Wrap},
};

use super::model::NotificationDialog;

impl Widget for &NotificationDialog {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let max_line_width: usize = self
            .body()
            .split_terminator('\n')
            .map(|line| line.len())
            .max()
            .unwrap_or(10);
        let popup_width = (max_line_width + 4)
            .min(area.width.saturating_sub(4) as usize)
            .max(30);
        let dictionary =
            hyphenation::Standard::from_embedded(hyphenation::Language::EnglishUS).unwrap();
        let wrap_options = textwrap::Options::new(popup_width).word_splitter(
            textwrap::word_splitters::WordSplitter::Hyphenation(dictionary),
        );
        let lines = textwrap::wrap(self.body(), wrap_options);
        let popup_height = (lines.len() + 4)
            .min(area.height.saturating_sub(4) as usize)
            .max(5);
        let popup_area = area.centered(
            Constraint::Length(popup_width as u16),
            Constraint::Length(popup_height as u16),
        );

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Span::from(format!(" {} ", self.title())).bold())
            .title_bottom(" Close <Enter/Space/Esc/q> ")
            .title_alignment(Alignment::Center)
            .set_style(self.style());

        let mut tui_lines = Vec::new();
        tui_lines.push(Line::raw(""));
        for line in lines {
            tui_lines.push(Line::raw(line));
        }

        let paragraph = Paragraph::new(tui_lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .centered();

        Clear.render(popup_area, buf);
        paragraph.render(popup_area, buf);
    }
}
