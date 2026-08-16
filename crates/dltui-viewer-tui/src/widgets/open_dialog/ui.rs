use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, List, ListItem, ListState, Paragraph, Widget},
};

use crate::widgets::open_dialog::model::{FileFinder, FileKind, OpenDialogState};

use super::model::OpenDialog;

impl Widget for &OpenDialog {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state() {
            OpenDialogState::Select => self.render_select(area, buf),
            OpenDialogState::FetchFile(finder) => self.render_fetch_file(finder, area, buf),
        }
    }
}

impl OpenDialog {
    fn render_select(&self, area: Rect, buf: &mut Buffer) {
        let popup_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(40));

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" Open File ")
            .title_bottom(
                " Validate <Enter/Space> | DLT Project <p> | Open DLT File <t> | Quit <q> ",
            )
            .title_alignment(Alignment::Center);

        let (dlp_style, dlt_style) = match self.selected() {
            FileKind::Dlp => (Style::new().add_modifier(Modifier::BOLD), Style::default()),
            FileKind::Dlt => (Style::default(), Style::new().add_modifier(Modifier::BOLD)),
        };

        let choices = Line::from(vec![
            Span::styled("DLT Project (.dlp)", dlp_style),
            Span::raw("      "),
            Span::styled("DLT Log File (.dlt)", dlt_style),
        ]);

        let question = Line::raw("What kind of file do you want to open ?");

        let paragraph = Paragraph::new(vec![question, choices])
            .block(block)
            .centered();

        Clear.render(popup_area, buf);
        paragraph.render(popup_area, buf);
    }

    fn render_fetch_file(&self, finder: &FileFinder, area: Rect, buf: &mut Buffer) {
        let popup_area = area.centered(Constraint::Percentage(70), Constraint::Percentage(70));

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(format!(" {} ", finder.current_dir().display()))
            .title_bottom(" Select <Enter> | Back <Esc> ")
            .title_alignment(Alignment::Center);
        let inner = block.inner(popup_area);

        Clear.render(popup_area, buf);
        block.render(popup_area, buf);

        let [query_area, list_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(inner);

        Paragraph::new(format!("> {}", finder.query()))
            .style(Style::new().add_modifier(Modifier::UNDERLINED))
            .render(query_area, buf);

        let items: Vec<ListItem> = finder
            .visible_entries()
            .map(|path| ListItem::new(finder.display_name(path)))
            .collect();

        let mut list_state = ListState::default().with_selected(Some(finder.selected_index()));
        let list = List::new(items).highlight_style(Style::new().add_modifier(Modifier::BOLD));

        ratatui::widgets::StatefulWidget::render(list, list_area, buf, &mut list_state);
    }
}
