use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

pub struct EcusViewerWidget<'a> {
    dlp: &'a dltui_viewer_dlp::dlt_project::DltProject,
    active: bool,
}

impl<'a> EcusViewerWidget<'a> {
    pub fn new(dlp: &'a dltui_viewer_dlp::dlt_project::DltProject, active: bool) -> Self {
        Self { dlp, active }
    }
}

impl Widget for EcusViewerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Span::from(" ECUs Viewer ").bold();
        let title = if self.active {
            title.underlined()
        } else {
            title
        };
        let footer = Span::from(" Back Home <Esc/^C> | Filters <f> ");
        let footer = if self.active {
            footer.bold()
        } else {
            footer.italic()
        };
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .title_bottom(footer)
            .title_alignment(Alignment::Left);

        let mut ecus = Vec::new();
        for ecu in &self.dlp.ecus {
            ecus.push(Line::from("")); // Separator line
            ecus.push(Line::from(vec![
                Span::styled(
                    format!("{}", ecu.id()),
                    Style::new().add_modifier(Modifier::BOLD),
                ),
                Span::from(format!(
                    " - {}",
                    ecu.description().unwrap_or("No description")
                )),
            ]));
            ecus.push(Line::from(Span::styled(
                format!("{}:{}", ecu.hostname().unwrap(), ecu.ip_port()),
                Style::new().add_modifier(Modifier::ITALIC),
            )));
        }

        let paragraph = Paragraph::new(ecus).block(block).left_aligned();

        Clear.render(area, buf);
        paragraph.render(area, buf);
    }
}
