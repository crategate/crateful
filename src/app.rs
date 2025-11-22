use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    DefaultTerminal,
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    // incoming path
    pub incoming: &'a Path,
    pub track_list: Vec<PathBuf>,
    pub index: usize,
    pub playing: PathBuf,
    pub length: Duration,
    pub progress: usize,
    pub music_player: Arc<Mutex<rodio::Sink>>,
    pub stream: rodio::OutputStream,
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
            index: 0,
            playing: PathBuf::new(),
            length: Duration::new(0, 0),
            progress: 0,
            music_player: Arc::new(Mutex::new(sink)),
            stream,
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
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::SaveTrack => self.save_track(),
                    AppEvent::DeleteTrack => self.delete_track(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }
}
