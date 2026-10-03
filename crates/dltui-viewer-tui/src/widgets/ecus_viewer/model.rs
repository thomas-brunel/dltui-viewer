use super::event::EcusViewerEvent;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy)]
pub enum EcusViewerKind {
    Yes,
    No,
}

#[derive(Debug, Default)]
pub struct EcusViewer;

impl EcusViewer {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<EcusViewerEvent> {
        match key_event.code {
            KeyCode::Esc => Some(EcusViewerEvent::BackHome),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                Some(EcusViewerEvent::BackHome)
            }
            _ => None,
        }
    }
}
