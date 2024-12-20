use std::path::PathBuf;

use eframe::egui;

use crate::{
    file_manager::FileManager,
    audio_player::AudioPlayer,
    ui::Ui
};

pub struct AppState {
    pub file_manager: FileManager,
    pub audio_player: AudioPlayer,
    ui: Ui,
}

impl AppState {
    pub fn new(music_folder: PathBuf) -> Self {
        let file_manager = FileManager::new(music_folder);
        let audio_player = AudioPlayer::new();
        let ui = Ui::new();

        // Start playing the first track if available
        if let Some(first_track) = file_manager.get_current_track() {
            audio_player.play_file(first_track);
        }

        AppState {
            file_manager,
            audio_player,
            ui,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Display current track information
        self.ui.show_track_info(ui, &self.file_manager.get_current_track());

        // Display next tracks
        let next_tracks = self.file_manager.get_next_tracks();
        self.ui.show_next_tracks(ui, &next_tracks);
    }
}
