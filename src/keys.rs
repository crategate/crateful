use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rodio::{Decoder, Source};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

use crate::app::App;
use crate::app::PauseMode;
use crate::env::Envs;
use crate::event::AppEvent;
use crate::event::WhichPath;
use crate::event::WhichPath::{PathA, PathD, PathG};
use ratatui_explorer::Input;

impl App {
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self.pause_mode {
            PauseMode::SaveSelect => match key_event.code {
                KeyCode::Up | KeyCode::Char('k') => self.explorer.handle(Input::Up).unwrap(),
                KeyCode::Down | KeyCode::Char('j') => self.explorer.handle(Input::Down).unwrap(),
                KeyCode::Left | KeyCode::Char('h') => self.explorer.handle(Input::Left).unwrap(),
                KeyCode::Right | KeyCode::Char('l') => self.explorer.handle(Input::Right).unwrap(),
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Enter => self.events.send(AppEvent::Select),
                _ => {}
            },
            PauseMode::MainMenu => match key_event.code {
                KeyCode::Char(' ') => self.events.send(AppEvent::Pause),
                KeyCode::Enter => self.events.send(AppEvent::SetPauseMode),
                KeyCode::Up | KeyCode::Char('k') => self.events.send(AppEvent::Up),
                KeyCode::Down | KeyCode::Char('j') => self.events.send(AppEvent::Down),
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                _ => {}
            },
            PauseMode::IncomingSelect => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Up | KeyCode::Char('k') => self.explorer.handle(Input::Up).unwrap(),
                KeyCode::Down | KeyCode::Char('j') => self.explorer.handle(Input::Down).unwrap(),
                KeyCode::Left | KeyCode::Char('h') => self.explorer.handle(Input::Left).unwrap(),
                KeyCode::Right | KeyCode::Char('l') => self.explorer.handle(Input::Right).unwrap(),
                KeyCode::Enter => self.events.send(AppEvent::Select),
                _ => {}
            },
            PauseMode::NotPaused => match key_event.code {
                KeyCode::Char('1') => self.events.send(AppEvent::Seek(1)),
                KeyCode::Char('2') => self.events.send(AppEvent::Seek(2)),
                KeyCode::Char('3') => self.events.send(AppEvent::Seek(3)),
                KeyCode::Char('4') => self.events.send(AppEvent::Seek(4)),
                KeyCode::Char('5') => self.events.send(AppEvent::Seek(5)),
                KeyCode::Char('6') => self.events.send(AppEvent::Seek(6)),
                KeyCode::Char('7') => self.events.send(AppEvent::Seek(7)),
                KeyCode::Char('8') => self.events.send(AppEvent::Seek(8)),
                KeyCode::Char('9') => self.events.send(AppEvent::Seek(9)),
                KeyCode::Char('a') => self.events.send(AppEvent::SaveTrack(PathA)),
                KeyCode::Char('d') => self.events.send(AppEvent::SaveTrack(PathD)),
                KeyCode::Char('g') => self.events.send(AppEvent::SaveTrack(PathG)),
                KeyCode::Backspace => self.events.send(AppEvent::DeleteTrack),
                KeyCode::Char(' ') => self.events.send(AppEvent::Pause),
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.events.send(AppEvent::Quit)
                }
                _ => {}
            },
            PauseMode::SelectError => match key_event.code {
                KeyCode::Char(' ') | KeyCode::Esc | KeyCode::Enter => {
                    self.events.send(AppEvent::AcceptError)
                }
                _ => {}
            },
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
        // when incoming isn't set, create the config system file. prompt user in pause menu
        if self.incoming.exists() {
            self.track_list = fs::read_dir(self.incoming.clone())
                //.unwrap_or_else(|a| fs::read_dir("../../").unwrap())
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect::<Vec<_>>();
            self.index = 0;
        } else {
            Envs::create_config();
            self.paused = true;
            self.pause_mode = PauseMode::IncomingSelect;
        }
    }

    pub fn start_playback(&mut self) {
        //self.load_tracks();
        let blank_bytes = include_bytes!("../blank.mp3");
        let no_track_andy = PathBuf::from("./blank.mp3");
        let blank = BufReader::new(File::open(&no_track_andy).unwrap());
        let file = BufReader::new(
            File::open(
                self.track_list
                    .get(self.index)
                    .unwrap_or_else(|| &no_track_andy),
            )
            .unwrap(),
        );
        let source = Decoder::try_from(file).unwrap_or_else(|a| Decoder::try_from(blank).unwrap());

        self.length = source.total_duration().expect("length read fail");
        self.music_player.lock().unwrap().append(source);
        self.music_player.lock().unwrap().play();
        if self.track_list.len() > 0 {
            self.playing = self.track_list.get(self.index).unwrap().to_path_buf();
        }
    }

    pub fn list_write(&mut self) {
        if self.track_list.len() > 0 {
            self.display_list = Vec::new();
            let _ = self
                .track_list
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    if i >= self.index {
                        self.display_list
                            .push(x.file_name().unwrap().to_str().unwrap().to_string())
                    }
                })
                .collect::<Vec<_>>();
        }
    }

    pub fn seek(&mut self, pos: u64) {
        if self.paused {
            return;
        }
        let percent = ((pos as f64 / 10.0) * self.length.as_secs() as f64).round();
        self.music_player.lock().unwrap().pause();
        self.music_player.lock().unwrap().clear();
        self.start_playback();
        let _ = self
            .music_player
            .lock()
            .unwrap()
            .try_seek(Duration::new(percent as u64, 0));
    }

    pub fn save_track(&mut self, which: WhichPath) {
        // move track file. increment index. Play next track.
        if self.paused {
            return;
        }
        let mut newpath;
        match which {
            PathA => newpath = self.save_path_a.as_ref().unwrap().clone(),
            PathD => newpath = self.save_path_d.as_ref().unwrap().clone(),
            PathG => newpath = self.save_path_g.as_ref().unwrap().clone(),
        }
        newpath.push(
            self.track_list
                .get(self.index)
                .unwrap()
                .file_name()
                .unwrap(),
        );
        let _ = fs::rename(self.track_list.get(self.index).unwrap(), newpath);
        self.index += 1;
        self.list_write();
        self.music_player.lock().unwrap().clear();
        self.start_playback();
    }

    pub fn delete_track(&mut self) {
        if self.paused {
            return;
        }
        // delete file. Increment index. Play next.
        self.music_player.lock().unwrap().clear();
        let _ = fs::remove_file(self.track_list.get(self.index).unwrap());
        self.index += 1;
        self.list_write();
        self.start_playback();
    }
    pub fn pause(&mut self) {
        self.pause_menu.select(Some(0));
        self.pause_mode = PauseMode::MainMenu;
        self.paused = !self.paused;
        if self.music_player.lock().unwrap().is_paused() {
            self.music_player.lock().unwrap().play();
            self.pause_mode = PauseMode::NotPaused;
        } else {
            self.music_player.lock().unwrap().pause();
        };
    }
    pub fn set_pause_mode(&mut self) {
        match self.pause_menu.selected().unwrap() {
            0 => {
                self.pause_mode = PauseMode::IncomingSelect;
                self.explorer_path = self.incoming.to_path_buf();
                let _ = self.explorer.set_cwd(self.incoming.clone());
                self.explorer_index = 0;
            }
            1 => {
                self.pause_mode = PauseMode::SaveSelect;
                self.explorer_path = self.save_path_a.as_ref().unwrap().clone();
                let _ = self
                    .explorer
                    .set_cwd(self.save_path_a.as_ref().unwrap().clone());
                self.explorer_index = 0;
            }
            2 => {
                self.pause_mode = PauseMode::NotPaused;
                self.pause();
            }
            _ => {}
        }
    }
    pub fn up(&mut self) {
        self.pause_menu.select_previous();
    }
    pub fn down(&mut self) {
        if self.pause_menu.selected().unwrap() < 2 {
            self.pause_menu.select_next();
        }
    }
    pub fn select(&mut self) {
        // check if selection is a directory, reject choice, display error message, & return if not
        if self.explorer.current().is_file() {
            self.pause_mode = PauseMode::SelectError;
            return;
        }
        match self.pause_mode {
            PauseMode::IncomingSelect => {
                self.incoming = self.explorer.current().path().to_path_buf();
                Envs::set_env(
                    "INCOMING_PATH",
                    self.explorer.current().path().to_str().unwrap(),
                );
                self.music_player.lock().unwrap().clear();
                self.paused = false;
                self.load_tracks();
                self.start_playback();
                self.list_write();
                self.pause_mode = PauseMode::NotPaused;
            }
            PauseMode::SaveSelect => {
                self.save_path_a = Some(self.explorer.current().path().to_path_buf());
                Envs::set_env(
                    "SAVE_PATH_A",
                    self.explorer.current().path().to_str().unwrap(),
                );
                self.paused = false;
                self.pause_mode = PauseMode::NotPaused;
                self.music_player.lock().unwrap().play();
            }
            _ => {}
        }
        self.paused = false;
    }

    pub fn set_items(&mut self) {}

    pub fn accept_erorr(&mut self) {
        self.pause_mode = PauseMode::MainMenu;
    }
}
