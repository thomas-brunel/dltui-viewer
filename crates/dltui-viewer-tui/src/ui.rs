use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Widget},
};

use crate::{
    app::{App, AppState},
    widgets::{ecus_viewer, frames_viewer},
};

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let home_active = matches!(self.state, AppState::Home);
        let title = if home_active {
            Span::from(" DLTUI VIEWER ").bold().underlined()
        } else {
            Span::from(" DLTUI VIEWER ").bold()
        };
        let footer = Span::from(
            " Open <o> | ECUs <e> | Frames <f> | Auto-Scroll <a> | Save <s> | Settings <c> | Quit <q> ",
        );
        let footer = if home_active {
            footer.bold()
        } else {
            footer.italic()
        };

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .title_top(Line::from(self.render_auto_scroll_badge()).right_aligned())
            .title_bottom(footer)
            .title_alignment(Alignment::Center);

        let inner = area.inner(Margin::new(2, 2));
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(inner);

        block.render(area, buf);
        ecus_viewer::EcusViewerWidget::new(
            &self.context.dlp,
            matches!(self.state, AppState::EcusViewer),
        )
        .render(inner_layout[0], buf);
        ratatui::widgets::StatefulWidget::render(
            frames_viewer::FramesViewerWidget::new(
                &self.context.dlp,
                self.context.dlt.as_ref(),
                matches!(self.state, AppState::FramesViewer),
            ),
            inner_layout[1],
            buf,
            &mut self.context.frames_viewer,
        );
    }
}

impl App {
    fn render_auto_scroll_badge(&self) -> Span<'static> {
        if self.context.dlp.settings.other().auto_scroll() {
            Span::from(" AUTO SCROLL [X] ")
                .add_modifier(Modifier::BOLD)
                .green()
        } else {
            Span::from(" AUTO SCROLL [ ] ")
        }
    }
}
