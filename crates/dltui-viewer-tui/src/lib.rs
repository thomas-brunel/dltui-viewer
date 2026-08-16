pub mod app;
pub mod event;
pub mod ui;
pub mod widgets;

pub fn start_tui() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app::App::new().run(terminal);
    ratatui::restore();
    result
}
