use eframe::egui;
use parking_lot::RwLock;
use std::sync::Arc;
use walkdir::WalkDir;

struct AppState {
    current_file_index: usize,
    files: Vec<String>,
    player: Option<playback_rs::Player>,
    playback_status: PlaybackStatus,
}

#[derive(Clone, Copy, PartialEq)]
enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

struct CratefulApp {
    state: Arc<RwLock<AppState>>,
}

impl CratefulApp {
    fn new() -> Self {
        let state = Arc::new(RwLock::new(AppState {
            current_file_index: 0,
            files: Vec::new(),
            player: None,
            playback_status: PlaybackStatus::Stopped,
        }));
        
        Self { state }
    }
    fn scan_directory(&self, path: &str) {
        let mut state = self.state.write();
        state.files.clear();

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ["mp3", "wav", "flac"].contains(&ext.as_str()) {
                    state.files.push(entry.path().to_string_lossy().into_owned());
                }
            }
        }
    }
}

impl eframe::App for CratefulApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = self.state.read();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Crateful - Audio Classifier");
            
            // File list display
            ui.separator();
            ui.label("Track List:");
            for (idx, file) in state.files.iter().enumerate() {
                ui.horizontal(|ui| {
                    if ui.selectable_label(idx == state.current_file_index, file).clicked() {
                        // Handle file selection
                    }
                });
            }
            
            // Playback controls
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Play/Pause (Space)").clicked() {
                    // Implement play/pause
                }
                
                // Will add more controls
            });
        });

        self.handle_hotkeys(ctx);
    }
}
// Hotkey handling implementation
impl CratefulApp {
    fn handle_hotkeys(&self, ctx: &egui::Context) {
        let input = ctx.input();
        let mut state = self.state.write();

        if input.key_pressed(egui::Key::Space) {
            self.toggle_playback();
        }

        // Number keys 1-9 for seeking
        for (key, percentage) in (egui::Key::Num1..=egui::Key::Num9).enumerate() {
            if input.key_pressed(percentage) {
                self.seek_to_percentage((key + 1) as f32 * 0.1);
            }
        }
    }

    fn toggle_playback(&self) {
        let mut state = self.state.write();
        match state.playback_status {
            PlaybackStatus::Playing => {
                if let Some(player) = &mut state.player {
                    player.pause();
                }
                state.playback_status = PlaybackStatus::Paused;
            }
            _ => {
                if state.player.is_none() {
                    // Initialize player with current file
                    if let Some(file) = state.files.get(state.current_file_index) {
                        let player = playback_rs::Player::new(file).unwrap();
                        player.play();
                        state.player = Some(player);
                        state.playback_status = PlaybackStatus::Playing;
                    }
                } else if let Some(player) = &mut state.player {
                    player.play();
                    state.playback_status = PlaybackStatus::Playing;
                }
            }
        }
    }

    fn seek_to_percentage(&self, percentage: f32) {
        let mut state = self.state.write();
        if let Some(player) = &mut state.player {
            if let Some(duration) = player.duration() {
                let target = duration.mul_f32(percentage);
                player.seek(target).unwrap();
            }
        }
    }
}
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Crateful",
        options,
        Box::new(|_cc| Box::new(CratefulApp::new())),
    );
}
