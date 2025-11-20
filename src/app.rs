use crate::event::{AppEvent, Event, EventHandler};
use crate::metadata::MetaData;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    DefaultTerminal,
};
use rodio::{Decoder, OutputStream, Sink};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

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

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('s') => self.events.send(AppEvent::SaveTrack),
            KeyCode::Char('k') => self.events.send(AppEvent::DeleteTrack),
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn load_tracks(&mut self) {
        // enumerate and save track list with pathes
    }
    pub fn start_playback(&mut self) {
        let file = BufReader::new(File::open(self.track_list.get(self.index).unwrap()).unwrap());
        let source = Decoder::new(file).unwrap();
        self.music_player.lock().unwrap().append(source);
        self.music_player.lock().unwrap().play();
        self.playing = self.track_list.get(self.index).unwrap().to_path_buf();
    }

    pub fn save_track(&mut self) {
        // move track file. increment index. Play next track.
        let mut newpath = PathBuf::from("../../Music/saved/");
        newpath.push(
            self.track_list
                .get(self.index)
                .unwrap()
                .file_name()
                .unwrap(),
        );
        fs::rename(self.track_list.get(self.index).unwrap(), newpath);
        self.index += 1;
        self.music_player.lock().unwrap().clear();
        self.start_playback();
    }

    pub fn delete_track(&mut self) {
        // delete file. Increment index. Play next.
        self.music_player.lock().unwrap().clear();
        fs::remove_file(self.track_list.get(self.index).unwrap());
        self.index += 1;
        self.start_playback();
    }
}
