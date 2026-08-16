use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Modifier, Style, Styled, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

use crate::widgets::confirm_dialog::model::ConfirmKind;

use super::model::ConfirmDialog;

impl Widget for &ConfirmDialog {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = area.centered(Constraint::Percentage(30), Constraint::Length(7));

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Span::from(format!(" {} ", self.title())).bold())
            .title_bottom(" Validate <Enter/Space> | Yes <y> | No <n> | Cancel <q> ")
            .title_alignment(Alignment::Center)
            .set_style(self.style());

        let (yes_style, no_style) = match self.selected() {
            ConfirmKind::Yes => (Style::new().add_modifier(Modifier::BOLD), Style::default()),
            ConfirmKind::No => (Style::default(), Style::new().add_modifier(Modifier::BOLD)),
        };

        let choices = Line::from(vec![
            Span::styled("Yes", yes_style),
            Span::raw("      "),
            Span::styled("No", no_style),
        ]);

        let question = Line::raw("Are you sure ?");

        let paragraph = Paragraph::new(vec![Line::raw(""), question, Line::raw(""), choices])
            .block(block)
            .centered();

        Clear.render(popup_area, buf);
        paragraph.render(popup_area, buf);
    }
}
