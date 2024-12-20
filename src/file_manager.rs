use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileManager {
    music_folder: PathBuf,
    audio_files: Vec<PathBuf>,
    current_index: usize,
}

impl FileManager {
    pub fn new(music_folder: PathBuf) -> Self {
        let mut manager = FileManager {
            music_folder: music_folder.clone(),
            audio_files: Vec::new(),
            current_index: 0,
        };
        manager.load_files(music_folder);
        manager
    }

    pub fn load_files(&mut self, folder: PathBuf) {
        self.audio_files.clear();
        for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
            if let Some(ext) = entry.path().extension() {
                if ext == "mp3" || ext == "wav" || ext == "flac" {
                    self.audio_files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    pub fn get_current_track(&self) -> Option<&PathBuf> {
        self.audio_files.get(self.current_index)
    }

    pub fn get_next_tracks(&self) -> Vec<&PathBuf> {
        let mut next_tracks = Vec::new();
        for i in self.current_index + 1..std::cmp::min(self.current_index + 4, self.audio_files.len()) {
            next_tracks.push(&self.audio_files[i]);
        }
        next_tracks
    }

    pub fn move_to_next_track(&mut self) {
        if self.current_index < self.audio_files.len() - 1 {
            self.current_index += 1;
        }
    }

    pub fn delete_current_track(&mut self) -> Result<(), std::io::Error> {
        if let Some(track_path) = self.get_current_track() {
            // Delete the file
            std::fs::remove_file(track_path)?;

            // Remove from the list and adjust current index
            self.audio_files.remove(self.current_index);
            if self.current_index >= self.audio_files.len() && self.current_index > 0 {
                self.current_index -= 1;
            }
        }
        Ok(())
    }

    pub fn move_current_track(&self, _destination_folder: &Path) {
        // TODO: Implement file moving logic
        // Consider using std::fs::rename to move the file
    }
}
