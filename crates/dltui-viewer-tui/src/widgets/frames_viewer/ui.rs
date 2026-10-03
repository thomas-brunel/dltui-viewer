use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Modifier, Style, Stylize},
    text::Span,
    widgets::{
        Block, BorderType, Clear, Paragraph, Row, StatefulWidget, Table, TableState, Widget,
    },
};

use super::model::FramesViewer;
use dltui_viewer_dlt::dlt_frame::DltFrame;

pub struct FramesViewerWidget<'a> {
    dlp: &'a dltui_viewer_dlp::dlt_project::DltProject,
    dlt: Option<&'a dltui_viewer_dlt::dlt_file::DltFile>,
    active: bool,
}

impl<'a> FramesViewerWidget<'a> {
    pub fn new(
        dlp: &'a dltui_viewer_dlp::dlt_project::DltProject,
        dlt: Option<&'a dltui_viewer_dlt::dlt_file::DltFile>,
        active: bool,
    ) -> Self {
        Self { dlp, dlt, active }
    }

    pub fn handle_header(&self) -> (Row<'static>, Vec<Constraint>) {
        let mut widths = Vec::new();
        let mut columns = Vec::new();

        if self.dlp.settings.table().show_index() {
            columns.push(Span::styled(
                "Index",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(6));
        }
        if self.dlp.settings.table().show_time() {
            columns.push(Span::styled(
                "Time",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(12));
        }
        if self.dlp.settings.table().show_timestamp() {
            columns.push(Span::styled(
                "Timestamp",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(25));
        }
        if self.dlp.settings.table().show_count() {
            columns.push(Span::styled(
                "Count",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(6));
        }
        if self.dlp.settings.table().show_ecu_id() {
            columns.push(Span::styled(
                "EcuID",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(6));
        }
        if self.dlp.settings.table().show_app_id() {
            columns.push(Span::styled(
                "ApID",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(6));
        }
        if self.dlp.settings.table().show_app_id_description() {
            columns.push(Span::styled(
                "ApID Description",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(18));
        }
        if self.dlp.settings.table().show_context_id() {
            columns.push(Span::styled(
                "CtID",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(6));
        }
        if self.dlp.settings.table().show_context_id_description() {
            columns.push(Span::styled(
                "CtID Description",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(18));
        }
        if self.dlp.settings.table().show_type() {
            columns.push(Span::styled(
                "Type",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(8));
        }
        if self.dlp.settings.table().show_subtype() {
            columns.push(Span::styled(
                "SubType",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(8));
        }
        if self.dlp.settings.table().show_mode() {
            columns.push(Span::styled(
                "Mode",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(6));
        }
        if self.dlp.settings.table().show_noar() {
            columns.push(Span::styled(
                "NoAR",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(6));
        }
        if self.dlp.settings.table().show_payload() {
            columns.push(Span::styled(
                "Payload",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Min(20));
        }
        if self.dlp.settings.table().show_arguments() {
            columns.push(Span::styled(
                "Arguments",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Min(20));
        }
        if self.dlp.settings.table().show_msg_id() {
            columns.push(Span::styled(
                "MsgID",
                Style::new().add_modifier(Modifier::BOLD),
            ));
            widths.push(Constraint::Length(6));
        }

        (Row::new(columns), widths)
    }

    pub fn handle_frame(&self, frame: &DltFrame) -> Row<'static> {
        let table = self.dlp.settings.table();
        let mut cells = Vec::new();

        let date_time = frame.get_date_time().ok();

        let time = date_time
            .as_ref()
            .map(|dt| dt.format("%H:%M:%S%.3f").to_string())
            .unwrap_or_else(|| "N/A".into());

        let timestamp = date_time
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string())
            .unwrap_or_else(|| "Invalid timestamp".into());

        let ecu_id = frame.get_ecu_id().unwrap_or_else(|_| "N/A".into());

        let (app_id, context_id, message_type, subtype) = frame
            .try_parse_extended_header()
            .ok()
            .flatten()
            .map(|(app_id, context_id, message_type, subtype)| {
                (app_id.0, context_id.0, message_type.0, subtype.0)
            })
            .unwrap_or_else(|| ("N/A".into(), "N/A".into(), "N/A".into(), "N/A".into()));

        let payload = frame
            .try_parse_payload()
            .unwrap_or_else(|_| "Failed to parse payload".into());

        if table.show_index() {
            cells.push(Span::from(frame.header_message_counter().to_string()));
        }
        if table.show_time() {
            cells.push(Span::from(time));
        }
        if table.show_timestamp() {
            cells.push(Span::from(timestamp));
        }
        if table.show_count() {
            cells.push(Span::from("N/A"));
        }
        if table.show_ecu_id() {
            cells.push(Span::from(ecu_id));
        }
        if table.show_app_id() {
            cells.push(Span::from(app_id.clone()));
        }
        if table.show_app_id_description() {
            cells.push(Span::from("N/A"));
        }
        if table.show_context_id() {
            cells.push(Span::from(context_id.clone()));
        }
        if table.show_context_id_description() {
            cells.push(Span::from("N/A"));
        }
        if table.show_type() {
            cells.push(Span::from(message_type));
        }
        if table.show_subtype() {
            cells.push(Span::from(subtype.clone()));
        }
        if table.show_mode() {
            cells.push(Span::from("N/A"));
        }
        if table.show_noar() {
            cells.push(Span::from("N/A"));
        }
        if table.show_payload() {
            cells.push(Span::from(payload));
        }
        if table.show_arguments() {
            cells.push(Span::from("N/A"));
        }
        if table.show_msg_id() {
            cells.push(Span::from("N/A"));
        }

        let row_style = match subtype.as_str() {
            "error" => Style::new().red().reversed(),
            "warn" => Style::new().yellow().reversed(),
            _ => Style::default(),
        };

        Row::new(cells).style(row_style)
    }
}

impl StatefulWidget for FramesViewerWidget<'_> {
    type State = FramesViewer;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = Span::from(" Frames Viewer ").bold();
        let title = if self.active {
            title.underlined()
        } else {
            title
        };
        let footer = Span::from(
            " Back <Esc/^C> | Up <k/↑> | Down <j/↓> | Page Up <Ctrl+u> | Page Down <Ctrl+d> | First <g> | Last <G> ",
        );
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

        let Some(dlt) = self.dlt else {
            let paragraph = Paragraph::new("No DLT file loaded")
                .block(block)
                .left_aligned();

            Clear.render(area, buf);
            paragraph.render(area, buf);
            return;
        };

        let (columns, widths) = self.handle_header();

        let visible_rows = area.height.saturating_sub(3) as usize;
        let start = state.scroll_offset().min(dlt.frame_count());
        let end = (start + visible_rows).min(dlt.frame_count());
        let mut rows = Vec::with_capacity(end.saturating_sub(start));

        for frame in &dlt.frames()[start..end] {
            let row = self.handle_frame(frame);
            rows.push(row);
        }

        let mut table_state = TableState::default();
        if !rows.is_empty() {
            table_state.select(Some(
                state.selected().saturating_sub(start).min(rows.len() - 1),
            ));
        }

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .header(columns)
            .block(block.clone())
            .row_highlight_style(Style::new().reversed())
            .highlight_symbol(">> ");

        Clear.render(area, buf);
        StatefulWidget::render(table, area, buf, &mut table_state);
    }
}
