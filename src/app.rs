use crate::env::Envs;

use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{DefaultTerminal, widgets::ListState};
use ratatui_explorer::FileExplorer;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    // incoming path
    pub incoming: PathBuf,
    pub save_path_a: Option<PathBuf>,
    pub save_path_d: Option<PathBuf>,
    pub save_path_g: Option<PathBuf>,
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
    SaveSelect(SavePath),
    IncomingSelect,
    SelectError,
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum SavePath {
    A,
    D,
    G,
}

impl Default for App {
    fn default() -> Self {
        let stream =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream.mixer());

        let incoming_from_env = Envs::read_env_var(String::from("INCOMING_PATH"))
            .unwrap_or_else(|_a| String::from("home/"));
        let save_a_env =
            Envs::read_env_var(String::from("SAVE_PATH_A")).unwrap_or_else(|_a| String::from(""));
        let save_d_env =
            Envs::read_env_var(String::from("SAVE_PATH_D")).unwrap_or_else(|_a| String::from(""));
        let save_g_env =
            Envs::read_env_var(String::from("SAVE_PATH_G")).unwrap_or_else(|_a| String::from(""));
        Self {
            running: true,
            events: EventHandler::new(),
            incoming: fs::canonicalize(PathBuf::from(incoming_from_env.clone()))
                .unwrap_or_else(|_a| PathBuf::from("")),
            track_list: Vec::new(),
            save_path_a: Some(
                fs::canonicalize(PathBuf::from(save_a_env)).unwrap_or_else(|_a| PathBuf::from("")),
            ),
            save_path_d: Some(
                fs::canonicalize(PathBuf::from(save_d_env)).unwrap_or_else(|_a| PathBuf::from("")),
            ),
            save_path_g: Some(
                fs::canonicalize(PathBuf::from(save_g_env)).unwrap_or_else(|_a| PathBuf::from("")),
            ),
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
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Envs::load_envs();
        Self::default()
    }
    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        if self.track_list.len() > 0 {}
        self.load_tracks();
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
                    AppEvent::SaveTrack(which) => self.save_track(which),
                    AppEvent::DeleteTrack => self.delete_track(),
                    AppEvent::Pause => self.pause(),
                    AppEvent::SetPauseMode => self.set_pause_mode(),
                    AppEvent::Quit => self.quit(),
                    AppEvent::Up => self.up(),
                    AppEvent::Down => self.down(),
                    AppEvent::Select => self.select(),
                    AppEvent::AcceptError => self.accept_erorr(),
                },
            }
        }
        Ok(())
    }
}
