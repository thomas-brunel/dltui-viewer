use super::event::ConfirmDialogEvent;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Style;

#[derive(Debug, Clone, Copy)]
pub enum ConfirmKind {
    Yes,
    No,
}

#[derive(Debug)]
pub struct ConfirmDialog {
    title: String,
    selected: ConfirmKind,
    style: Style,
}

impl ConfirmDialog {
    pub fn new(title: &str, style: Style) -> Self {
        Self {
            title: title.into(),
            selected: ConfirmKind::No,
            style,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn selected(&self) -> ConfirmKind {
        self.selected
    }

    pub fn style(&self) -> Style {
        self.style
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<ConfirmDialogEvent> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q' | 'Q') => Some(ConfirmDialogEvent::No),
            KeyCode::Tab
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Char('h' | 'H')
            | KeyCode::Char('l' | 'L') => {
                self.selected = match self.selected {
                    ConfirmKind::Yes => ConfirmKind::No,
                    ConfirmKind::No => ConfirmKind::Yes,
                };
                None
            }
            KeyCode::Char('y' | 'Y') => Some(ConfirmDialogEvent::Yes),
            KeyCode::Char('n' | 'N') => Some(ConfirmDialogEvent::No),
            KeyCode::Enter | KeyCode::Char(' ') => match self.selected {
                ConfirmKind::Yes => Some(ConfirmDialogEvent::Yes),
                ConfirmKind::No => Some(ConfirmDialogEvent::No),
            },
            _ => None,
        }
    }
}
