use std::fs::File;
use std::path::{Path, PathBuf};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct MetaData {
    src: File,
    mss: MediaSourceStream,
}

impl MetaData {
    pub fn new(song: PathBuf) -> Self {
        let songsrc = File::open(song).expect("needs clean song file...");
        Self {
            src: songsrc.try_clone().expect("where file..."),
            mss: MediaSourceStream::new(Box::new(songsrc), Default::default()),
        }
    }
}
