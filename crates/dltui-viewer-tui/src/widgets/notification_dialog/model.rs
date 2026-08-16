use super::event::NotificationDialogEvent;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Style;

#[derive(Debug)]
pub struct NotificationDialog {
    title: String,
    body: String,
    style: Style,
}

impl NotificationDialog {
    pub fn new(title: &str, body: &str, style: Style) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            style,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn style(&self) -> Style {
        self.style
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<NotificationDialogEvent> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q' | 'Q') | KeyCode::Enter | KeyCode::Char(' ') => {
                Some(NotificationDialogEvent::Ok)
            }
            _ => None,
        }
    }
}
