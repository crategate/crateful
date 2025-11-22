use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::app::App;
use crate::event::{AppEvent, Event, EventHandler};

impl App<'_> {
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

        self.length = source.total_duration().expect("length read fail");
        self.music_player.lock().unwrap().append(source);
        self.music_player.lock().unwrap().play();
        self.playing = self.track_list.get(self.index).unwrap().to_path_buf();
    }
    pub fn seek(&mut self, pos: u8) {
        self.music_player
            .lock()
            .unwrap()
            .try_seek(Duration::new(20, 0));
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
