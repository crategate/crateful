use eframe::egui::{self, Color32, Layout};
use eframe::epaint::Vec2;
use std::path::PathBuf;

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

    pub fn show_track_info(
        &mut self,
        ui: &mut egui::Ui,
        track_path: &Option<PathBuf>,
        _bpm: u16,
        _key: &str,
    ) {
        let track_name = match track_path {
            Some(path) => path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("Unknown"),
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
                            button = button.fill(Color32::from_rgb(100, 100, 150));
                            // Highlight color
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
                let track_name = track
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or("Unknown");
                ui.label(track_name);
            }
        });
    }

    fn add_folder_assignment_dialog(&mut self, ui: &mut egui::Ui) {
        // Placeholder for a dialog to add a new folder assignment
        // This could be a simple text input for the folder path and hotkey
        // For now, we'll just add a dummy entry
        self.folder_hotkeys
            .push(("A".to_string(), "FolderA".to_string()));
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
#[derive(PartialEq)]
pub struct FrameDemo {
    frame: egui::Frame,
}

impl Default for FrameDemo {
    fn default() -> Self {
        Self {
            frame: egui::Frame::new()
                .inner_margin(12)
                .outer_margin(24)
                .corner_radius(14)
                .shadow(egui::Shadow {
                    offset: [8, 12],
                    blur: 16,
                    spread: 0,
                    color: egui::Color32::from_black_alpha(180),
                })
                .fill(egui::Color32::from_rgba_unmultiplied(97, 0, 255, 128))
                .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY)),
        }
    }
}

impl crate::Demo for FrameDemo {
    fn name(&self) -> &'static str {
        "▣ Frame"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                use crate::View as _;
                self.ui(ui);
            });
    }
}

impl crate::View for FrameDemo {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(&mut self.frame);

                ui.add_space(8.0);
                ui.set_max_width(ui.min_size().x);
                ui.vertical_centered(|ui| egui::reset_button(ui, self, "Reset"));
            });

            ui.separator();

            ui.vertical(|ui| {
                // We want to paint a background around the outer margin of the demonstration frame, so we use another frame around it:
                egui::Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .corner_radius(ui.visuals().widgets.noninteractive.corner_radius)
                    .show(ui, |ui| {
                        self.frame.show(ui, |ui| {
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                            ui.label(egui::RichText::new("Content").color(egui::Color32::WHITE));
                        });
                    });
            });
        });

        ui.set_max_width(ui.min_size().x);
        ui.separator();
        ui.vertical_centered(|ui| ui.add(crate::egui_github_link_file!()));
    }
}
