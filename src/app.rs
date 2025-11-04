use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    DefaultTerminal,
};
use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
// think we need regular mutex, as tokio yeilds lock of Mutex to the executing thread
// meaning the track would play through instead of interrupt as we need.
use tokio::sync::{broadcast, Mutex};
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    // incoming path
    pub incoming: &'a Path,
    pub track_list: Vec<PathBuf>,
    pub playing: PathBuf,
    pub music_player: rodio::Sink,
    pub stream: rodio::OutputStream,
}

impl Default for App<'_> {
    fn default() -> Self {
        let stream =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream.mixer());

        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            incoming: Path::new("../../Music/incoming/"),
            track_list: fs::read_dir("../../Music/incoming")
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect::<Vec<_>>(),
            playing: PathBuf::new(),
            music_player: sink,
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
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
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
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
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

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
        self.start_playback();
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn load_tracks(&mut self) {
        // enumerate and save track list with pathes
        // self.track_list = self.incoming
        //        for x in init_tracks {
        //           self.track_list.push(x.unwrap().path())
        //      }
    }
    pub fn start_playback(&mut self) {
        //self.music_player.append(coder);

        self.music_player.play();
    }

    pub fn save_track(&mut self) {
        // move track file. Play next track. Modify tracklist
    }
    pub fn delete_track(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}
