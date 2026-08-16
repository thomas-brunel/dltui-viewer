use super::event::OpenDialogEvent;
use crossterm::event::{KeyCode, KeyEvent};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Dlp,
    Dlt,
}

impl FileKind {
    fn extension(self) -> &'static str {
        match self {
            FileKind::Dlp => "dlp",
            FileKind::Dlt => "dlt",
        }
    }
}

#[derive(Debug)]
pub struct FileFinder {
    current_dir: PathBuf,
    kind: FileKind,
    query: String,
    all_files: Vec<PathBuf>,
    search_started: bool,
    search_rx: Option<mpsc::Receiver<PathBuf>>,
    results: Vec<PathBuf>,
    selected: usize,
}

impl FileFinder {
    pub fn new(current_dir: PathBuf, kind: FileKind) -> Self {
        let mut finder = Self {
            current_dir,
            kind,
            query: String::new(),
            all_files: Vec::new(),
            search_started: false,
            search_rx: None,
            results: Vec::new(),
            selected: 0,
        };
        finder.refresh();
        finder
    }

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn visible_entries(&self) -> impl Iterator<Item = &PathBuf> {
        self.results.iter()
    }

    pub fn display_name(&self, path: &Path) -> String {
        if self.current_dir.parent() == Some(path) {
            return "../".to_string();
        }
        if self.query.is_empty() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
            if path.is_dir() {
                format!("{name}/")
            } else {
                name.to_string()
            }
        } else {
            path.strip_prefix(&self.current_dir)
                .unwrap_or(path)
                .display()
                .to_string()
        }
    }

    fn refresh(&mut self) {
        self.results = if self.query.is_empty() {
            self.browse_entries()
        } else {
            self.start_search_if_needed();
            self.filter_all_files()
        };
        self.selected = self.selected.min(self.results.len().saturating_sub(1));
    }

    pub fn poll_search(&mut self) {
        let Some(rx) = &self.search_rx else {
            return;
        };
        let mut received_any = false;
        loop {
            match rx.try_recv() {
                Ok(path) => {
                    self.all_files.push(path);
                    received_any = true;
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.search_rx = None;
                    break;
                }
            }
        }
        if received_any && !self.query.is_empty() {
            self.results = self.filter_all_files();
            self.selected = self.selected.min(self.results.len().saturating_sub(1));
        }
    }

    fn browse_entries(&self) -> Vec<PathBuf> {
        let extension = self.kind.extension();
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&self.current_dir)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_dir()
                    || path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
            })
            .collect();
        entries.sort();
        if let Some(parent) = self.current_dir.parent() {
            entries.insert(0, parent.to_path_buf());
        }
        entries
    }

    fn start_search_if_needed(&mut self) {
        if self.search_started {
            return;
        }
        self.search_started = true;

        let (tx, rx) = mpsc::channel();
        let root = self.current_dir.clone();
        let extension = self.kind.extension();
        thread::spawn(move || Self::walk_files(&root, extension, &tx));
        self.search_rx = Some(rx);
    }

    fn walk_files(root: &Path, extension: &str, tx: &mpsc::Sender<PathBuf>) {
        let mut pending = vec![root.to_path_buf()];
        while let Some(dir) = pending.pop() {
            let Ok(read_dir) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in read_dir.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    pending.push(path);
                } else if path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
                    && tx.send(path).is_err()
                {
                    return;
                }
            }
        }
    }

    fn filter_all_files(&self) -> Vec<PathBuf> {
        let query = self.query.to_lowercase();
        let mut results: Vec<PathBuf> = self
            .all_files
            .iter()
            .filter(|path| {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                name.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();
        results.sort();
        results
    }

    fn push_char(&mut self, c: char) {
        self.query.push(c);
        self.refresh();
    }

    fn pop_char(&mut self) {
        self.query.pop();
        self.refresh();
    }

    fn move_selection(&mut self, delta: isize) {
        if self.results.is_empty() {
            return;
        }
        let len = self.results.len() as isize;
        let next = (self.selected as isize + delta).rem_euclid(len);
        self.selected = next as usize;
    }

    fn selected_path(&self) -> Option<PathBuf> {
        self.results.get(self.selected).cloned()
    }

    fn confirm(&mut self) -> Option<PathBuf> {
        let path = self.selected_path()?;
        if path.is_dir() {
            self.current_dir = path;
            self.query.clear();
            self.search_started = false;
            self.search_rx = None;
            self.all_files.clear();
            self.refresh();
            None
        } else {
            Some(path)
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent, kind: FileKind) -> Option<OpenDialogEvent> {
        match key.code {
            KeyCode::Char(c) => {
                self.push_char(c);
                None
            }
            KeyCode::Backspace => {
                self.pop_char();
                None
            }
            KeyCode::Up => {
                self.move_selection(-1);
                None
            }
            KeyCode::Down => {
                self.move_selection(1);
                None
            }
            KeyCode::Enter => self
                .confirm()
                .map(|path| OpenDialogEvent::FileChosen(kind, path)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum OpenDialogState {
    Select,
    FetchFile(FileFinder),
}

#[derive(Debug)]
pub struct OpenDialog {
    selected: FileKind,
    state: OpenDialogState,
}

impl OpenDialog {
    pub fn new() -> Self {
        Self {
            selected: FileKind::Dlp,
            state: OpenDialogState::Select,
        }
    }

    pub fn selected(&self) -> FileKind {
        self.selected
    }

    pub fn state(&self) -> &OpenDialogState {
        &self.state
    }

    /// Drains any results the background finder has found since the last call.
    pub fn poll_search(&mut self) {
        if let OpenDialogState::FetchFile(finder) = &mut self.state {
            finder.poll_search();
        }
    }

    fn enter_finder(&mut self, kind: FileKind) {
        self.selected = kind;
        self.state = OpenDialogState::FetchFile(FileFinder::new(
            std::env::current_dir().unwrap_or_default(),
            kind,
        ));
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<OpenDialogEvent> {
        if matches!(self.state, OpenDialogState::Select) {
            self.handle_select_key(key_event)
        } else {
            self.handle_finder_key(key_event)
        }
    }

    fn handle_select_key(&mut self, key_event: KeyEvent) -> Option<OpenDialogEvent> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q' | 'Q') => Some(OpenDialogEvent::Quit),
            KeyCode::Tab
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Char('h' | 'H')
            | KeyCode::Char('l' | 'L') => {
                self.selected = match self.selected {
                    FileKind::Dlp => FileKind::Dlt,
                    FileKind::Dlt => FileKind::Dlp,
                };
                None // handled internally, nothing to bubble up
            }
            KeyCode::Char('p' | 'P') => {
                self.enter_finder(FileKind::Dlp);
                None
            }
            KeyCode::Char('t' | 'T') => {
                self.enter_finder(FileKind::Dlt);
                None
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let kind = self.selected;
                self.enter_finder(kind);
                None
            }
            _ => None,
        }
    }

    fn handle_finder_key(&mut self, key_event: KeyEvent) -> Option<OpenDialogEvent> {
        if key_event.code == KeyCode::Esc {
            self.state = OpenDialogState::Select;
            return None;
        }
        let selected_kind = self.selected;
        let OpenDialogState::FetchFile(finder) = &mut self.state else {
            return None;
        };
        finder.handle_key_event(key_event, selected_kind)
    }
}
