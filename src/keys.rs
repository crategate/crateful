use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::app::App;
use crate::app::PauseMode;
use crate::event::{AppEvent, Event, EventHandler, WhichPath};
use ratatui_explorer::Input;

impl App<'_> {
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self.pause_mode {
            PauseMode::SaveSelect => match key_event.code {
                KeyCode::Up | KeyCode::Char('k') => self.events.send(AppEvent::PathUp),
                KeyCode::Down | KeyCode::Char('j') => self.events.send(AppEvent::PathDown),
                KeyCode::Left | KeyCode::Char('h') => self.events.send(AppEvent::PathParent),
                KeyCode::Right | KeyCode::Char('l') => self.events.send(AppEvent::PathChild),
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Enter => self.events.send(AppEvent::SetPath(WhichPath::PathA)),
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
                KeyCode::Up | KeyCode::Char('k') => self.events.send(AppEvent::PathUp),
                KeyCode::Down | KeyCode::Char('j') => self.events.send(AppEvent::PathDown),
                KeyCode::Left | KeyCode::Char('h') => self.events.send(AppEvent::PathParent),
                KeyCode::Right | KeyCode::Char('l') => self.events.send(AppEvent::PathChild),
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
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
                KeyCode::Char('s') => self.events.send(AppEvent::SaveTrack),
                KeyCode::Char('k') => self.events.send(AppEvent::DeleteTrack),
                KeyCode::Char(' ') => self.events.send(AppEvent::Pause),
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.events.send(AppEvent::Quit)
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
    }
    pub fn start_playback(&mut self) {
        let file = BufReader::new(File::open(self.track_list.get(self.index).unwrap()).unwrap());
        let source = Decoder::try_from(file).unwrap();

        self.length = source.total_duration().expect("length read fail");
        self.music_player.lock().unwrap().append(source);
        self.music_player.lock().unwrap().play();
        self.playing = self.track_list.get(self.index).unwrap().to_path_buf();
    }

    pub fn list_write(&mut self) {
        self.display_list = Vec::new();
        let _ = self
            .track_list
            .iter()
            .enumerate()
            .map(|(i, x)| {
                if i >= self.index {
                    self.display_list
                        .push(x.to_str().unwrap().to_string()[21..].to_string())
                }
            })
            .collect::<Vec<_>>();
    }

    pub fn seek(&mut self, pos: u64) {
        if self.paused {
            return;
        }
        let percent = ((pos as f64 / 10.0) * self.length.as_secs() as f64).round();
        // self.playing = PathBuf::from(percent.to_string());
        self.music_player.lock().unwrap().pause();
        self.music_player.lock().unwrap().clear();
        self.start_playback();
        let _ = self
            .music_player
            .lock()
            .unwrap()
            .try_seek(Duration::new(percent as u64, 0));
    }

    pub fn save_track(&mut self) {
        // move track file. increment index. Play next track.
        if self.paused {
            return;
        }
        let mut newpath = PathBuf::from("../../Music/saved/");
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
        } else {
            self.music_player.lock().unwrap().pause();
        };
    }
    pub fn set_pause_mode(&mut self) {
        match self.pause_menu.selected().unwrap() {
            0 => {
                self.pause_mode = PauseMode::IncomingSelect;
                self.explorer_path = self.incoming.to_path_buf();
                self.explorer.set_cwd(self.incoming.to_path_buf());
                self.explorer_index = 0;
                self.set_items();
            }
            1 => {
                self.pause_mode = PauseMode::SaveSelect;
                self.explorer_path = self.save_path_a.to_path_buf();
                self.explorer.set_cwd(self.save_path_a.clone());
                self.explorer_index = 0;
                self.set_items();
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
        dbg!(self.save_path_a.clone());
    }
    pub fn select(&mut self) {
        //self.pause_mode = self.pause_menu.selected().unwrap();
    }

    pub fn path_up(&mut self) {
        self.explorer.handle(Input::Up).unwrap();
    }
    pub fn path_down(&mut self) {
        //if self.explorer_index < self.explorer_items.len() {
        //    self.explorer
        //        .set_selected_idx(self.explorer.selected_idx() + 1);
        //    self.explorer_index += 1;
        //}
        self.explorer.handle(Input::Down).unwrap();
    }
    pub fn set_items(&mut self) {
        //     let (mut dirs, mut none_dirs): (Vec<_>, Vec<_>) = std::fs::read_dir(&self.explorer_path)?
        //         .filter_map(|entry| {
        //             let entry = entry.ok()?;
        //             let path = entry.path();
        //             let metadata = path.metadata().ok();
        //             let file_type = metadata.as_ref().map(|f| f.file_type());
        //             let is_dir = file_type.is_some_and(|f| f.is_dir());

        //             let name = entry.file_name().to_string_lossy().into_owned();
        //             let name = if is_dir { format!("{}/", name) } else { name };

        //             let is_hidden = {
        //                 #[cfg(unix)]
        //                 {
        //                     name.starts_with('.')
        //                 }

        //                 #[cfg(windows)]
        //                 {
        //                     use std::os::windows::fs::MetadataExt;
        //                     const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
        //                     metadata.is_some_and(|f| f.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0)
        //                 }
        //             };

        //             let file = AFile {
        //                 name,
        //                 path,
        //                 is_dir,
        //                 is_hidden,
        //                 file_type,
        //             };
        //             if !self.show_hidden && file.is_hidden() {
        //                 None
        //             } else {
        //                 Some(file)
        //             }
        //         })
        //         .partition(AFile::is_dir);

        //     dirs.sort_unstable_by(|f1, f2| f1.name.cmp(&f2.name));
        //     none_dirs.sort_unstable_by(|f1, f2| f1.name.cmp(&f2.name));

        //     if let Some(parent) = self.cwd.parent() {
        //         let mut files = Vec::with_capacity(1 + dirs.len() + none_dirs.len());

        //         files.push(AFile {
        //             name: "../".to_owned(),
        //             path: parent.to_path_buf(),
        //             is_dir: true,
        //             is_hidden: false,
        //             file_type: None,
        //         });

        //         files.extend(dirs);
        //         files.extend(none_dirs);

        //         self.explorer.files = files;
        //     } else {
        //         let mut files = Vec::with_capacity(dirs.len() + none_dirs.len());

        //         files.extend(dirs);
        //         files.extend(none_dirs);

        //         self.files = files;
        //     };

        //     Ok(())
        //   let mut items = Vec::new();
        //   for entry in fs::read_dir(self.explorer_path.clone()).expect("read failure") {
        //       items.push(entry.unwrap())
        //   }
        //   items.sort_by_key(|dir| dir.path());
        //   self.explorer_items = items;
    }
    pub fn path_parent(&mut self) {
        self.explorer.handle(Input::Left).unwrap();
        //        let parent = self.explorer_path.parent();
        //
        //        if let Some(parent) = parent {
        //            self.explorer_path = self.explorer_path.parent().unwrap().to_path_buf();
        //            self.explorer.set_cwd(self.explorer_path.to_path_buf());
        //            self.set_items();
        //        }
        //        self.explorer_index = 0;
    }
    pub fn path_child(&mut self) {
        self.explorer.handle(Input::Right).unwrap();

        //if self.explorer_index < 1 {
        //    return;
        //}
        //let mut contents = Vec::new();
        //for entry in fs::read_dir(self.explorer_path.clone()).expect("failed to read") {
        //    contents.push(entry.unwrap());
        //}
        //contents.sort_by_key(|dir| dir.path());
        //dbg!(&contents[self.explorer_index - 1]);
        //if contents[self.explorer_index - 1].path().is_dir() {
        //    self.explorer_path = contents[self.explorer_index - 1].path();
        //    self.explorer.set_cwd(self.explorer_path.to_path_buf());
        //    self.explorer_index = 0;
        //    self.set_items();
        //} else {
        //}
        //self.explorer_path = self.explorer_path;
    }
    pub fn set_path(&mut self, which: WhichPath) {
        // check if directory before setting
        dbg!("PUSH");
        // match which {WhichPath::PathA => }
    }
}
