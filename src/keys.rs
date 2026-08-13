use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rodio::source::Source;
use rodio::Decoder;

use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::time::Instant;
use walkdir::WalkDir;

use crate::App;
use crate::app::Indicator;
use crate::app::PauseMode;
use crate::app::SavePath;
use crate::app::SavePath::{A, D, G};
use crate::env::Envs;
use crate::event::Amp;
use crate::event::AppEvent;
use ratatui_explorer::Input;

pub trait FileExtension {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool;
}

impl<P: AsRef<Path>> FileExtension for P {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(extension) = self.as_ref().extension().and_then(OsStr::to_str) {
            return extensions
                .iter()
                .any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }

        false
    }
}
impl App {
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match self.pause_mode {
                PauseMode::SaveSelect(_path) => match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') => self.explorer.handle(Input::Up).unwrap(),
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.explorer.handle(Input::Down).unwrap()
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.explorer.handle(Input::Left).unwrap()
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.explorer.handle(Input::Right).unwrap()
                    }
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
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.explorer.handle(Input::Down).unwrap()
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.explorer.handle(Input::Left).unwrap()
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.explorer.handle(Input::Right).unwrap()
                    }
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
                    KeyCode::Char('h') | KeyCode::Left => self.events.send(AppEvent::SkipBack),
                    KeyCode::Char('l') | KeyCode::Right => self.events.send(AppEvent::SkipForward),
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.events.send(AppEvent::Volume(Amp::Down))
                    }
                    KeyCode::Char('k') | KeyCode::Up => self.events.send(AppEvent::Volume(Amp::Up)),
                    KeyCode::Char('a') => self.events.send(AppEvent::SaveTrack(A)),
                    KeyCode::Char('d') => self.events.send(AppEvent::SaveTrack(D)),
                    KeyCode::Char('g') => self.events.send(AppEvent::SaveTrack(G)),
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
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        // Clear the visual action indicator once its deadline has passed.
        if let Some(deadline) = self.indicator_deadline
            && Instant::now() >= deadline {
                self.visual_action_indicator = None;
                self.indicator_deadline = None;
            }
        // Skip the progress update when no track with a known length is loaded.
        // (This also replaces the old `incoming.exists()` stat on every tick:
        // an unset incoming folder means length stays zero.)
        let secs = self.length.as_secs();
        if secs == 0 {
            return;
        }
        let point = self.music_player.lock().unwrap().get_pos().as_secs() as f64;
        let percent = (point / secs as f64) * 100.0;
        if 100.0 > percent && percent > 0.0 {
            self.progress = percent;
            self.format_time = format!(
                "{}:{:0>2} out of {}:{:0>2}",
                (point as u64 / 60),
                (point as u64 % 60),
                secs / 60,
                secs % 60
            )
        }
    }

    /// Sets the visual action indicator and schedules it to be cleared after
    /// `timeout_ms` milliseconds. The actual clearing happens in `tick()`, so the
    /// event loop is never blocked waiting for the timeout to elapse.
    pub fn set_indicator(&mut self, indicator: Indicator, timeout_ms: u64) {
        self.visual_action_indicator = Some(indicator);
        self.indicator_deadline = Some(Instant::now() + Duration::from_millis(timeout_ms));
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn load_tracks(&mut self) {
        // enumerate and save track list with pathes
        // when incoming isn't set, create the config system file. prompt user in pause menu
        if self.incoming.exists() {
            self.track_list = WalkDir::new(self.incoming.clone())
                .into_iter()
                //.unwrap_or_else(|a| fs::read_dir("../../").unwrap())
                .filter_map(|e| {
                    if e.as_ref()
                        .ok()
                        .unwrap()
                        .path()
                        .has_extension(&["mp3", "wav", "flac"])
                    {
                        e.ok()
                    } else {
                        None
                    }
                })
                .map(|e| e.path().to_path_buf())
                .collect::<Vec<_>>();
            self.index = 0;
        } else {
            Envs::create_config();
            self.paused = true;
            self.pause_mode = PauseMode::IncomingSelect;
        }
    }

    pub fn start_playback(&mut self) {
        // Skip over tracks that can't be opened or decoded instead of crashing.
        // Removing them keeps `index` pointing at the next playable track.
        let mut removed_bad_track = false;
        while let Some(path) = self.track_list.get(self.index) {
            let Ok(file) = File::open(path) else {
                self.track_list.remove(self.index);
                removed_bad_track = true;
                continue;
            };
            match Decoder::try_from(BufReader::new(file)) {
                Ok(source) => {
                    self.length = source.total_duration().unwrap_or(Duration::ZERO);
                    self.music_player.lock().unwrap().append(source);
                    self.playing = path.clone();
                    self.music_player.lock().unwrap().play();
                    if removed_bad_track {
                        self.list_write();
                    }
                    return;
                }
                Err(_) => {
                    self.track_list.remove(self.index);
                    removed_bad_track = true;
                }
            }
        }
        // No playable tracks left: keep the sink alive with a silent file.
        let blank_bytes = include_bytes!("../blank.mp3");
        let blank_curs = BufReader::new(Cursor::new(blank_bytes));
        if let Ok(blank_source) = Decoder::try_from(blank_curs) {
            self.music_player.lock().unwrap().append(blank_source);
        }
        self.playing = PathBuf::new();
        self.length = Duration::ZERO;
        self.music_player.lock().unwrap().play();
        if removed_bad_track {
            self.list_write();
        }
    }

    pub fn volume(&mut self, amp: Amp) {
        let vol_now = self.music_player.lock().unwrap().volume();
        self.set_indicator(Indicator::Volume, 500);
        match amp {
            Amp::Up => {
                if vol_now < 1.15 {
                    self.music_player.lock().unwrap().set_volume(vol_now + 0.05);
                }
            }
            Amp::Down => {
                if vol_now > 0.15 {
                    self.music_player.lock().unwrap().set_volume(vol_now - 0.04);
                }
            }
        }
    }

    pub fn list_write(&mut self) {
        self.display_list = self
            .track_list
            .iter()
            .skip(self.index)
            .filter_map(|x| x.file_name())
            .map(|name| name.to_string_lossy().into_owned())
            .collect();
    }

    pub fn seek(&mut self, pos: u64) {
        if self.paused {
            return;
        }
        self.set_indicator(Indicator::Scrubbed, 200);
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
    pub fn skip_back(&mut self) {
        if self.paused {
            return;
        }
        self.set_indicator(Indicator::Scrubbed, 200);
        let current_pos = self.music_player.lock().unwrap().get_pos();
        self.music_player.lock().unwrap().pause();
        self.music_player.lock().unwrap().clear();
        self.start_playback();
        let _ = self
            .music_player
            .lock()
            .unwrap()
            .try_seek(current_pos.saturating_sub(Duration::from_secs(2)));
    }

    pub fn skip_forward(&mut self) {
        if self.paused {
            return;
        }
        self.set_indicator(Indicator::Scrubbed, 200);
        let current_pos = self.music_player.lock().unwrap().get_pos();
        let _ = self
            .music_player
            .lock()
            .unwrap()
            .try_seek(current_pos.saturating_add(Duration::from_secs(2)));
    }

    pub fn save_track(&mut self, which: SavePath) {
        // move track file. increment index. Play next track.
        if self.paused || self.track_list.is_empty() {
            return;
        }
        let mut newpath;
        match which {
            SavePath::A => {
                newpath = self.save_path_a.as_ref().unwrap().clone();
                self.set_indicator(Indicator::SavedA, 300)
            }
            SavePath::D => {
                newpath = self.save_path_d.as_ref().unwrap().clone();
                self.set_indicator(Indicator::SavedD, 300)
            }
            SavePath::G => {
                newpath = self.save_path_g.as_ref().unwrap().clone();
                self.set_indicator(Indicator::SavedG, 300)
            }
        }
        if newpath.as_os_str().is_empty() {
            self.pause();
            match which {
                SavePath::A => self.pause_menu.select(Some(1)),
                SavePath::D => self.pause_menu.select(Some(2)),
                SavePath::G => self.pause_menu.select(Some(3)),
            }
            self.pause_mode = PauseMode::SaveSelect(which);
            return ;
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
        if self.display_list.len() < 5 {
            self.load_tracks();
            self.list_write();
            self.index = 0;
        }
        self.start_playback();
    }

    pub fn delete_track(&mut self) {
        if self.paused || self.track_list.is_empty() {
            return;
        }
        self.set_indicator(Indicator::Deleted, 400);
        // delete file. Increment index. Play next.
        self.music_player.lock().unwrap().clear();
        let _ = fs::remove_file(self.track_list.get(self.index).unwrap());
        self.index += 1;
        self.list_write();
        if self.display_list.len() < 5 {
            self.load_tracks();
            self.list_write();
            self.index = 0;
        }
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
                self.pause_mode = PauseMode::SaveSelect(A);
                self.explorer_path = self.save_path_a.as_ref().unwrap().clone();
                let _ = self
                    .explorer
                    .set_cwd(self.save_path_a.as_ref().unwrap().clone());
                self.explorer_index = 0;
            }
            2 => {
                self.pause_mode = PauseMode::SaveSelect(D);
                self.explorer_path = self.save_path_d.as_ref().unwrap().clone();
                let _ = self
                    .explorer
                    .set_cwd(self.save_path_d.as_ref().unwrap().clone());
                self.explorer_index = 0;
            }
            3 => {
                self.pause_mode = PauseMode::SaveSelect(G);
                self.explorer_path = self.save_path_g.as_ref().unwrap().clone();
                let _ = self
                    .explorer
                    .set_cwd(self.save_path_g.as_ref().unwrap().clone());
                self.explorer_index = 0;
            }
            4 => {
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
        if self.pause_menu.selected().unwrap() < 4 {
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
                self.load_tracks();
                self.paused = false;
                self.start_playback();
                self.list_write();
                self.pause_mode = PauseMode::NotPaused;
            }
            PauseMode::SaveSelect(save_path) => {
                let this_path = Some(self.explorer.current().path().to_path_buf());
                match save_path {
                    A => {
                        Envs::set_env(
                            "SAVE_PATH_A",
                            self.explorer.current().path().to_str().unwrap(),
                        );
                        self.save_path_a = this_path;
                    }
                    D => {
                        Envs::set_env(
                            "SAVE_PATH_D",
                            self.explorer.current().path().to_str().unwrap(),
                        );
                        self.save_path_d = this_path;
                    }
                    G => {
                        Envs::set_env(
                            "SAVE_PATH_G",
                            self.explorer.current().path().to_str().unwrap(),
                        );
                        self.save_path_g = this_path;
                    }
                }

                self.paused = false;
                self.pause_mode = PauseMode::NotPaused;
                self.music_player.lock().unwrap().play();
            }
            _ => {}
        }
        self.paused = false;
    }

    pub fn set_items(&mut self) {}

    pub fn accept_error(&mut self) {
        self.pause_mode = PauseMode::MainMenu;
    }
}
