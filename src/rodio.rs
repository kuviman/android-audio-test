use std::rc::Rc;

use rodio::Source;

pub struct Sound {
    audio: Audio,
    buffer: Vec<u8>,
}

impl Sound {
    pub fn play(&self) {
        let decoder = rodio::Decoder::new(std::io::Cursor::new(self.buffer.clone())).unwrap();
        self.audio
            .stream_handle
            .play_raw(decoder.convert_samples())
            .unwrap();
    }
}

#[derive(Clone)]
pub struct Audio {
    stream_handle: rodio::OutputStreamHandle,
    stream: Rc<rodio::OutputStream>,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        Self {
            stream: Rc::new(stream),
            stream_handle,
        }
    }
    pub fn decode(&self, buffer: Vec<u8>) -> Sound {
        Sound {
            buffer,
            audio: self.clone(),
        }
    }
}
