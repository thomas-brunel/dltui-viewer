use super::model::FileKind;

/// Open File events.
#[derive(Clone, Debug)]
pub enum OpenDialogEvent {
    /// A file was picked in the finder.
    FileChosen(FileKind, std::path::PathBuf),
    /// Quit the window.
    Quit,
}
