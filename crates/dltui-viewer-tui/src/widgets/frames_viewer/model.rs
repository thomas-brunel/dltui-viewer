use super::event::FramesViewerEvent;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Default)]
pub struct FramesViewer {
    selected: usize,
    scroll_offset: usize,
}

impl FramesViewer {
    pub fn new() -> Self {
        Self {
            selected: 0,
            scroll_offset: 0,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<FramesViewerEvent> {
        match key_event.code {
            KeyCode::Esc => Some(FramesViewerEvent::BackHome),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                Some(FramesViewerEvent::BackHome)
            }
            KeyCode::Char('k' | 'K') | KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                self.selected = self.selected.saturating_sub(1);
                Some(FramesViewerEvent::ScrollUp)
            }
            KeyCode::Char('j' | 'J') | KeyCode::Down => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                self.selected = self.selected.saturating_add(1);
                Some(FramesViewerEvent::ScrollDown)
            }
            KeyCode::Char('u' | 'U') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
                self.selected = self.selected.saturating_sub(10);
                Some(FramesViewerEvent::ScrollUp)
            }
            KeyCode::Char('d' | 'D') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.scroll_offset = self.scroll_offset.saturating_add(10);
                self.selected = self.selected.saturating_add(10);
                Some(FramesViewerEvent::ScrollDown)
            }
            KeyCode::Char('g') => {
                self.scroll_offset = 0;
                self.selected = 0;
                Some(FramesViewerEvent::SelectFrame(0))
            }
            KeyCode::Char('G') => {
                self.scroll_offset = usize::MAX;
                self.selected = usize::MAX;
                Some(FramesViewerEvent::SelectFrame(usize::MAX))
            }
            _ => None,
        }
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }
}
