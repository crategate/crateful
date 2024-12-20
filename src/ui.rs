use std::path::PathBuf;
use eframe::egui;

pub struct Ui {
}

impl Ui {
    pub fn new() -> Self {
        Ui {
        }
    }

    pub fn show_track_info(&self, ui: &mut egui::Ui, track_path: &Option<PathBuf>) {
        ui.label("Current Track:");
        match track_path {
            Some(path) => {
                ui.label(path.file_name().unwrap().to_str().unwrap_or("Unknown"));
            }
            None => {
                ui.label("No track loaded");
            }
        }
    }

    pub fn show_next_tracks(&self, ui: &mut egui::Ui, next_tracks: &Vec<&PathBuf>) {
        ui.label("Next Tracks:");
        for track in next_tracks {
            ui.label(track.file_name().unwrap().to_str().unwrap_or("Unknown"));
        }
    }
}
