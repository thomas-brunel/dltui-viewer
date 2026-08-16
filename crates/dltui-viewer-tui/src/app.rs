use crate::{
    event::{AppEvent, Event, EventHandler},
    widgets,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

// Application State.
#[derive(Debug)]
pub enum AppState {
    Home,
    Open(widgets::open_dialog::model::OpenDialog),
    Notification(widgets::notification_dialog::model::NotificationDialog),
    Quit(widgets::confirm_dialog::model::ConfirmDialog),
}

#[derive(Debug, Default)]
pub struct AppContext {
    // dltui_settings: dltui_viewer_core::dltui_settings::DltuiSettings>, // TODO configuration specific of DLTUI-Viewer
    pub dlp: dltui_viewer_dlp::dlt_project::DltProject,
    pub dlt: Option<dltui_viewer_dlt::dlt_file::DltFile>,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    // Current State.
    pub state: AppState,
    // Application context
    pub context: AppContext,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            state: AppState::Home,
            context: AppContext::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                frame.render_widget(&self, frame.area());
                if let AppState::Open(open_dialog) = &self.state {
                    frame.render_widget(open_dialog, frame.area()); // popup renders on top
                }
                if let AppState::Notification(notification_dialog) = &self.state {
                    frame.render_widget(notification_dialog, frame.area());
                }
                if let AppState::Quit(quit_dialog) = &self.state {
                    frame.render_widget(quit_dialog, frame.area());
                }
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    match &mut self.state {
                        AppState::Home => self.handle_key_event(key_event)?,
                        AppState::Open(open_dialog) => {
                            if let Some(open_event) = open_dialog.handle_key_event(key_event) {
                                self.handle_open_event(open_event);
                            }
                        }
                        AppState::Notification(notification_dialog) => {
                            if let Some(_notification_event) =
                                notification_dialog.handle_key_event(key_event)
                            {
                                self.home()
                            }
                        }
                        AppState::Quit(quit_dialog) => {
                            if let Some(quit_event) = quit_dialog.handle_key_event(key_event) {
                                match quit_event {
                                    widgets::confirm_dialog::event::ConfirmDialogEvent::Yes => {
                                        self.quit()
                                    }
                                    widgets::confirm_dialog::event::ConfirmDialogEvent::No => {
                                        self.home()
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::OpenFile => {
                    self.state = AppState::Open(widgets::open_dialog::model::OpenDialog::new());
                }
                AppEvent::NotifyError((title, body)) => {
                    self.state = AppState::Notification(
                        widgets::notification_dialog::model::NotificationDialog::new(
                            &title,
                            &body,
                            ratatui::style::Style::new().red(),
                        ),
                    )
                }
                AppEvent::Quit => {
                    self.state = AppState::Quit(widgets::confirm_dialog::model::ConfirmDialog::new(
                        "Quit Application",
                        ratatui::style::Style::new().red(),
                    ))
                }
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char('o' | 'O') => self.events.send(AppEvent::OpenFile),
            KeyCode::Char('e' | 'E') => {}
            KeyCode::Char('f' | 'F') => {}
            KeyCode::Char('a' | 'A') => {
                self.context.dlp.settings.other.auto_scroll =
                    !self.context.dlp.settings.other.auto_scroll;
            }
            KeyCode::Char('s' | 'S') => {}
            KeyCode::Char('c' | 'C') => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        if let AppState::Open(open_state) = &mut self.state {
            open_state.poll_search();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn home(&mut self) {
        self.state = AppState::Home;
    }

    fn handle_open_event(&mut self, event: widgets::open_dialog::event::OpenDialogEvent) {
        use widgets::open_dialog::event::OpenDialogEvent;
        match event {
            // TODO: hand `kind`/`path` off to the actual .dlp/.dlt loader once wired up.
            OpenDialogEvent::FileChosen(kind, path) => {
                match kind {
                    widgets::open_dialog::model::FileKind::Dlp => {
                        match dltui_viewer_dlp::dlt_project::DltProject::open(&path) {
                            Ok(dlp) => self.context.dlp = dlp,
                            Err(e) => self.events.send(AppEvent::NotifyError((
                                "DLT PROJECT FILE".into(),
                                format!("Failed to open file `{:?}` - {:?}", &path, e),
                            ))),
                        }
                    }
                    widgets::open_dialog::model::FileKind::Dlt => {
                        match dltui_viewer_dlt::dlt_file::DltFile::open(&path) {
                            Ok(dlt) => self.context.dlt = Some(dlt),
                            Err(e) => self.events.send(AppEvent::NotifyError((
                                "DLT LOG FILE".into(),
                                format!("Failed to open file: `{:?}` - {:?}", &path, e),
                            ))),
                        }
                    }
                }
                self.home()
            }
            OpenDialogEvent::Quit => self.home(),
        }
    }
}
