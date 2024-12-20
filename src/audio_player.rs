use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, Source};

pub struct AudioPlayer {
    sink: Sink,
    _stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        AudioPlayer {
            sink,
            _stream: stream,
        }
    }

    pub fn play_file(&self, file_path: &PathBuf) {
        // Stop currently playing file, if any
        self.sink.stop();

        // Load the new file
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        // Append the new source and play
        self.sink.append(source);
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn seek_to(&self, _duration: Duration) {
        // Unsupported for now
    }
}
