mod audio_player;
mod file_manager;
mod ui;
mod app_state;

use std::path::PathBuf;

use eframe::egui;
use rfd::FileDialog;
use app_state::AppState;

fn main() -> Result<(), eframe::Error> {
    // Request folder from user at startup
    let music_folder = FileDialog::new()
        .set_title("Select Music Folder")
        .pick_folder();

    match music_folder {
        Some(folder) => {
            let options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
                ..Default::default()
            };
            eframe::run_native(
                "Music Sorter",
                options,
                Box::new(|_cc| Box::new(MusicSorterApp::new(folder))),
            )
        }
        None => {
            // Handle case where no folder is selected
            // For now, just exit
            eprintln!("No folder selected. Exiting.");
            Ok(())
        }
    }
}

#[derive(Default)]
pub struct MusicSorterApp {
    app_state: AppState,
}

impl MusicSorterApp {
    fn new(music_folder: PathBuf) -> Self {
        Self {
            app_state: AppState::new(music_folder),
        }
    }
}

impl eframe::App for MusicSorterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.app_state.ui(ui);
        });
    }
}
