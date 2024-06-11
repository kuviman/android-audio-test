use std::rc::Rc;

use web_audio_api::{
    context::BaseAudioContext,
    node::{AudioNode, AudioScheduledSourceNode},
};

pub struct Sound {
    audio: Audio,
    buffer: web_audio_api::AudioBuffer,
}

impl Sound {
    pub fn play(&self) {
        let mut node = self.audio.inner.create_buffer_source();
        node.set_buffer(self.buffer.clone());
        node.connect(&self.audio.inner.destination());
        node.start();
    }
}

#[derive(Clone)]
pub struct Audio {
    inner: Rc<web_audio_api::context::AudioContext>,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(web_audio_api::context::AudioContext::new(Default::default())),
        }
    }
    pub fn decode(&self, buffer: Vec<u8>) -> Sound {
        let buffer = self
            .inner
            .decode_audio_data_sync(std::io::Cursor::new(buffer))
            .unwrap();
        Sound {
            audio: self.clone(),
            buffer,
        }
    }
}
