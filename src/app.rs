use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Alignment, Constraint, Layout, Offset, Rect},
    widgets::{Block, Borders, ListState},
};
use ratatui_explorer::{FileExplorer, Theme};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{self, Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    // incoming path
    pub incoming: &'a Path,
    pub save_path_a: PathBuf,
    pub track_list: Vec<PathBuf>,
    pub display_list: Vec<String>,
    pub index: usize,
    pub playing: PathBuf,
    pub paused: bool,
    pub pause_menu: ListState,
    pub pause_mode: PauseMode,
    pub explorer: FileExplorer,
    pub explorer_path: PathBuf,
    pub explorer_index: usize,
    pub explorer_items: Vec<std::fs::DirEntry>,
    pub length: Duration,
    pub progress: usize,
    pub music_player: Arc<Mutex<rodio::Sink>>,
    pub stream: rodio::OutputStream,
    pub volume: f32,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub enum PauseMode {
    #[default]
    NotPaused,
    MainMenu,
    SaveSelect,
    IncomingSelect,
}

impl Default for App<'_> {
    fn default() -> Self {
        let stream =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream.mixer());

        Self {
            running: true,
            events: EventHandler::new(),
            incoming: Path::new("../../Music/incoming/"),
            track_list: fs::read_dir("../../Music/incoming")
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect::<Vec<_>>(),
            save_path_a: fs::canonicalize(PathBuf::from("../../Music/saved")).unwrap(),
            display_list: Vec::new(),
            index: 0,
            playing: PathBuf::new(),
            paused: false,
            pause_menu: ListState::default().with_selected(Some(0)),
            pause_mode: PauseMode::NotPaused,
            explorer: FileExplorer::new().unwrap(),
            explorer_path: PathBuf::new(),
            explorer_items: Vec::new(),
            explorer_index: 0,
            length: Duration::new(0, 0),
            progress: 0,
            music_player: Arc::new(Mutex::new(sink)),
            stream,
            volume: 1.0,
        }
    }
}
impl App<'_> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.start_playback();
        self.list_write();
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Seek(num) => self.seek(num),
                    AppEvent::SaveTrack => self.save_track(),
                    AppEvent::DeleteTrack => self.delete_track(),
                    AppEvent::Pause => self.pause(),
                    AppEvent::SetPauseMode => self.set_pause_mode(),
                    AppEvent::Quit => self.quit(),
                    AppEvent::Up => self.up(),
                    AppEvent::Down => self.down(),
                    AppEvent::Select => self.select(),
                    AppEvent::PathDown => self.path_down(),
                    AppEvent::PathUp => self.path_up(),
                    AppEvent::SetPath(which) => self.set_path(which),
                    AppEvent::PathParent => self.path_parent(),
                    AppEvent::PathChild => self.path_child(),
                },
            }
        }
        Ok(())
    }
}
