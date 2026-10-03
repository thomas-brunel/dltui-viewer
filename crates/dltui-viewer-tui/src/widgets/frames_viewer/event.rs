/// Frames Viewer events.
#[derive(Clone, Debug)]
pub enum FramesViewerEvent {
    /// Back Home event.
    BackHome,
    /// Select frame event.
    SelectFrame(usize),
    /// Scroll up event.
    ScrollUp,
    /// Scroll down event.
    ScrollDown,
}
