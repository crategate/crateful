use eframe::egui;
use parking_lot::Mutex;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;
//use symphonia::core::io::MediaSourceStream;
// use symphonia::core::probe::Hint;

struct AudioState {
    current_file: Option<String>,
    playhead_seconds: f64,
    duration_seconds: f64,
    is_playing: bool,
}

struct CratefulApp {
    files: Vec<String>,
    audio_state: Arc<Mutex<AudioState>>,
    audio_thread: Option<std::thread::JoinHandle<()>>,
}

impl CratefulApp {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            audio_state: Arc::new(Mutex::new(AudioState {
                current_file: None,
                playhead_seconds: 0.0,
                duration_seconds: 0.0,
                is_playing: false,
            })),
            audio_thread: None,
        }
    }

    fn scan_directory(&mut self, path: &str) {
        self.files.clear();

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ["mp3", "wav", "flac"].contains(&ext.as_str()) {
                    self.files.push(entry.path().to_string_lossy().into_owned());
                }
            }
        }
    }
}

impl eframe::App for CratefulApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Crateful - Audio Classifier");

            // File list
            ui.separator();
            ui.label("Track List:");
            for file in &self.files {
                ui.label(file);
            }

            // Playback controls
            ui.separator();
            let audio_state = self.audio_state.lock();
            ui.label(format!(
                "Status: {} | {:.1}/{:.1} seconds",
                if audio_state.is_playing { "▶" } else { "⏸" },
                audio_state.playhead_seconds,
                audio_state.duration_seconds
            ));
        });

        // Handle hotkeys
        //        if ctx.input().key_pressed(egui::Key::Space) {
        //    let mut audio_state = self.audio_state.lock();
        //    audio_state.is_playing = !audio_state.is_playing;
        // }
    }
}

fn main() -> anyhow::Result<()> {
    let mut app = CratefulApp::new();
    // Get directory from CLI args
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args[1]);

    assert!(!Path::new(args[1].to_string()).exists());
    if args.len() > 1 {
        app.scan_directory(&args[1]);
    }

    let options = eframe::NativeOptions::default();
    eframe::run_native("Crateful", options, Box::new(|_cc| Box::new(app)));

    Ok(())
}
