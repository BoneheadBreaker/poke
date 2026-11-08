use crate::event::{AppEvent, Event, EventHandler};
use std::{
    fs::{self},
    io,
    path::Path,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edit::edit_file;
use ratatui::{DefaultTerminal, widgets::ListState};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    // Folders in current directory
    pub folders: Vec<String>,
    pub folder_list: ListState,
    pub file_list: ListState,
    pub files: Vec<String>,
    pub focus_element: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            folders: Vec::new(),
            files: Vec::new(),
            folder_list: ListState::default(),
            file_list: ListState::default(),
            focus_element: "Folder".to_string(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_folders(&mut self, path: &Path) -> io::Result<()> {
        self.folders.clear();
        self.files.clear();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            let name = entry.file_name().to_string_lossy().into_owned();

            if kind.is_dir() {
                self.folders.push(name);
            } else if kind.is_file() {
                self.files.push(name);
            }
        }

        self.folders.sort_unstable();
        self.files.sort_unstable();
        Ok(())
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
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
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),

            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit);
            }

            KeyCode::Up => {
                if self.focus_element == "Folder" {
                    self.folder_list.select_next();
                } else {
                    self.file_list.select_next();
                }
            }

            KeyCode::Down => {
                if self.focus_element == "Folder" {
                    self.folder_list.select_previous();
                } else {
                    self.file_list.select_previous();
                }
            }

            KeyCode::Enter => {
                if self.focus_element == "Folder" {
                    if let Some(folder) = self
                        .folder_list
                        .selected()
                        .and_then(|index| self.folders.get(index))
                    {
                        std::env::set_current_dir(folder)?;
                        self.load_folders(Path::new("."))?;
                        self.folder_list.select_first();
                        self.file_list.select_first();
                    }
                } else if self.focus_element == "File" {
                    if let Some(name) = self
                        .file_list
                        .selected()
                        .and_then(|index| self.files.get(index))
                    {
                        edit_file(name)?;
                    }
                }
            }

            KeyCode::Char('a' | 'A') => {
                self.focus_element = "Folder".to_string();
            }

            KeyCode::Char('d' | 'D') => {
                self.focus_element = "File".to_string();
            }

            _ => {}
        }

        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

}
