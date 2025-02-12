use std::path::PathBuf;
use eframe::egui::{self, Layout, Color32};
use eframe::epaint::Vec2;

#[derive(Default)]
pub struct UiState {
    selected_folder_index: Option<usize>,
    delete_pressed: bool,
}

pub struct MusicSorterUi {
    pub state: UiState,
    pub folder_hotkeys: Vec<(String, String)>, // (hotkey, folder_name)
}

impl Default for MusicSorterUi {
    fn default() -> Self {
        Self {
            state: UiState::default(),
            folder_hotkeys: Vec::new(),
        }
    }
}

impl MusicSorterUi {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show_track_info(&mut self, ui: &mut egui::Ui, track_path: &Option<PathBuf>, _bpm: u16, _key: &str) {
        let track_name = match track_path {
            Some(path) => path.file_name().unwrap_or_default().to_str().unwrap_or("Unknown"),
            None => "No track loaded",
        };

        ui.vertical_centered(|ui| {
            ui.heading(track_name);
            ui.label("BPM: --");
            ui.label("Key: --");
        });
    }

    pub fn show_progress_bar(&self, ui: &mut egui::Ui, progress: f32) {
        let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(true);
        ui.add(progress_bar);
    }

    pub fn show_folder_assignment(&mut self, ui: &mut egui::Ui) {
        if self.folder_hotkeys.is_empty() {
            if ui.button("Add Folder").clicked() {
                self.add_folder_assignment_dialog(ui);
            }
        } else {
            ui.horizontal_wrapped(|ui| {
                for (index, (hotkey, folder_name)) in self.folder_hotkeys.iter().enumerate() {
                    let button_text = format!("{} ({})", folder_name, hotkey);
                    let mut button = egui::Button::new(button_text);
    
                    if let Some(selected_index) = self.state.selected_folder_index {
                        if selected_index == index {
                            button = button.fill(Color32::from_rgb(100, 100, 150)); // Highlight color
                        }
                    }
    
                    if ui.add(button).clicked() {
                        self.handle_folder_hotkey_click(index);
                    }
                }
    
                if ui.button("+").clicked() {
                    self.add_folder_assignment_dialog(ui);
                }
            });
        }
    }

    pub fn show_next_tracks(&self, ui: &mut egui::Ui, next_tracks: &[&PathBuf]) {
        ui.label("Up Next:");
        egui::ScrollArea::vertical().show(ui, |ui| {
            for track in next_tracks {
                let track_name = track.file_name().unwrap_or_default().to_str().unwrap_or("Unknown");
                ui.label(track_name);
            }
        });
    }

    fn add_folder_assignment_dialog(&mut self, ui: &mut egui::Ui) {
        // Placeholder for a dialog to add a new folder assignment
        // This could be a simple text input for the folder path and hotkey
        // For now, we'll just add a dummy entry
        self.folder_hotkeys.push(("A".to_string(), "FolderA".to_string()));
    }

    fn handle_folder_hotkey_click(&mut self, index: usize) {
        if self.state.selected_folder_index == Some(index) {
            // Second tap, assign track to folder
            // TODO: Implement track assignment
            self.state.selected_folder_index = None;
        } else {
            // First tap, select folder
            self.state.selected_folder_index = Some(index);
        }
    }
}
