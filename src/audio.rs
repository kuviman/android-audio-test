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
        log::debug!("creating buffer source node");
        let mut node = self.audio.inner.create_buffer_source();
        log::debug!("settting the buffer");
        node.set_buffer(self.buffer.clone());
        log::debug!("connecting to destination");
        node.connect(&self.audio.inner.destination());
        log::debug!("starting");
        node.start();
        log::debug!("audio should be playing now...");
    }
}

#[derive(Clone)]
pub struct Audio {
    inner: Rc<web_audio_api::context::AudioContext>,
}

impl Audio {
    pub fn new() -> Self {
        log::debug!("before audio context creation");
        let context = web_audio_api::context::AudioContext::new(Default::default());
        log::debug!("after audio context creation");
        Self {
            inner: Rc::new(context),
        }
    }
    pub fn decode(&self, buffer: Vec<u8>) -> Sound {
        log::debug!("before decoding audio");
        let buffer = self
            .inner
            .decode_audio_data_sync(std::io::Cursor::new(buffer))
            .unwrap();
        log::debug!("after decoding audio");
        Sound {
            audio: self.clone(),
            buffer,
        }
    }
}
