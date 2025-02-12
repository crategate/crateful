use std::path::PathBuf;

use eframe::egui;

use crate::{
    file_manager::FileManager,
    audio_player::AudioPlayer,
    ui::MusicSorterUi
};

pub struct AppState {
    pub file_manager: FileManager,
    pub audio_player: AudioPlayer,
    ui: MusicSorterUi,
    track_progress: f32,
}

impl AppState {
    pub fn new(music_folder: PathBuf) -> Self {
        let file_manager = FileManager::new(music_folder);
        let audio_player = AudioPlayer::new();
        let ui = MusicSorterUi::new();

        // Start playing the first track if available
        if let Some(first_track) = file_manager.get_current_track() {
            audio_player.play_file(first_track);
        }

        AppState {
            file_manager,
            audio_player,
            ui,
            track_progress: 0.0,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Top section: Track info
        ui.vertical(|ui| {
            // Track info
            ui.group(|ui| {
                self.ui.show_track_info(ui, &self.file_manager.get_current_track().cloned(), 120, "C Major");
            });
            // Add some vertical spacing
            ui.add_space(10.0);
            // Progress bar
            self.ui.show_progress_bar(ui, self.track_progress); // Always show the progress bar
        });

        // Add some vertical spacing
        ui.add_space(10.0);

        // Middle section: Folder assignment
        self.ui.show_folder_assignment(ui);

        // Bottom section: Up next list
        let next_tracks = self.file_manager.get_next_tracks();
        self.ui.show_next_tracks(ui, &next_tracks);
    }
}


impl Default for AppState {
    fn default() -> Self {
        let music_folder = PathBuf::new(); // You can provide a default path if needed
        let file_manager = FileManager::new(music_folder);
        let audio_player = AudioPlayer::new();
        let ui = Ui::new();

        Self {
            file_manager,
            audio_player,
            ui,
        }
    }
}
