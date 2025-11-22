use std::fs::File;
use std::path::{Path, PathBuf};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct MetaData {
    track: String,
}

impl MetaData {
    pub fn new(song: PathBuf) -> Self {
        let songsrc = File::open(song).expect("needs clean song file...");
        let mss = MediaSourceStream::new(Box::new(songsrc), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("mp3");
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();
        let mut probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("unsupported format");

        Self {
            track: "metadata".to_string(),
        }
    }
}
