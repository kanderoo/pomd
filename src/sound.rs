use std::{fs::File, io::BufReader};

use rodio::{Decoder, OutputStream, OutputStreamHandle, PlayError, Source};

pub struct SoundHandler {
    _stream: OutputStream,
    handle: OutputStreamHandle
}

impl SoundHandler {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        Self {
            _stream: stream,
            handle
        }
    }

    pub fn play_complete(&self) -> Result<(), PlayError> {
        let file = BufReader::new(File::open("/usr/share/sounds/freedesktop/stereo/complete.oga").unwrap());

        let source = Decoder::new(file).unwrap();

        self.handle.play_raw(source.convert_samples())?;

        Ok(())
    }
}