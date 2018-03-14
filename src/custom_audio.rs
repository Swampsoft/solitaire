use std::fmt;
use std::io;
use std::mem;
use std::path;

use rodio;

use ggez::{Context, GameResult};
use ggez::audio::*;

/// This is a modified copy of ggez::audio::Source without the annoying habbit to queue
/// rapidly played sounds.
pub struct Source {
    data: io::Cursor<SoundData>,
    sink: rodio::Sink,
}

impl Source {
    /// Create a new Source from the given file.
    pub fn new<P: AsRef<path::Path>>(context: &mut Context, path: P) -> GameResult<Self> {
        let path = path.as_ref();
        let data = {
            let file = &mut context.filesystem.open(path)?;
            SoundData::from_read(file)?
        };
        Source::from_data(context, data)
    }

    /// Creates a new Source using the given SoundData object.
    pub fn from_data(_context: &mut Context, data: SoundData) -> GameResult<Self> {
        // MB: ugly but necessary because the endpoint is private within the audio context
        #[allow(deprecated)]
        let endpoint = rodio::get_default_endpoint().unwrap();
        let sink = rodio::Sink::new(&endpoint);
        let cursor = io::Cursor::new(data);
        Ok(Source {
            data: cursor,
            sink: sink,
        })
    }

    /// Plays the Source.
    pub fn play(&mut self) -> GameResult<()> {
        // Creating a new Decoder each time seems a little messy,
        // since it may do checking and data-type detection that is
        // redundant, but it's not super expensive.
        // See https://github.com/ggez/ggez/issues/98 for discussion
        let cursor = self.data.clone();
        let decoder = rodio::Decoder::new(cursor)?;
        #[allow(deprecated)]
        let endpoint = rodio::get_default_endpoint().unwrap();
        let sink = rodio::Sink::new(&endpoint);
        mem::replace(&mut self.sink, sink).detach();
        self.sink.append(decoder);
        Ok(())
    }

    /// Returns whether or not the source is stopped
    /// -- that is, has no more data to play.
    pub fn stopped(&self) -> bool {
        self.sink.empty()
    }

    /// Sets the current volume
    pub fn set_volume(&mut self, value: f32) {
        self.sink.set_volume(value)
    }

    /// Get whether or not the source is paused
    pub fn paused(&self) -> bool {
        self.sink.is_paused()
    }

    /// Get whether or not the source is playing (ie, not paused
    /// and not stopped)
    pub fn playing(&self) -> bool {
        !self.paused() && !self.stopped()
    }
}

impl fmt::Debug for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Audio source: {:p}>", self)
    }
}
