use std::sync::Mutex;

use winit::{
    event::{ElementState, KeyEvent, Touch, TouchPhase, WindowEvent},
    window::{Window, WindowAttributes},
};

struct App {
    window: Option<Window>,
    audio: geng_audio::Audio,
    sound: geng_audio::Sound,
}

impl App {
    fn new() -> Self {
        let audio = geng_audio::Audio::new().unwrap();
        let data = load_audio_file();
        let sound = futures::executor::block_on(audio.decode(data)).unwrap();
        println!("app initialized");
        Self {
            window: None,
            audio,
            sound,
        }
    }

    fn create_window(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window.replace(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        println!("window created");
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.create_window(event_loop);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        println!("{event:?}");
        if let WindowEvent::Touch(Touch {
            phase: TouchPhase::Started,
            ..
        })
        | WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                },
            ..
        }
        | WindowEvent::MouseInput {
            state: ElementState::Pressed,
            ..
        } = event
        {
            self.sound.play();
        }
    }
}

#[cfg(target_os = "android")]
static APP: Mutex<Option<android_activity::AndroidApp>> = Mutex::new(None);

#[cfg(not(target_os = "android"))]
fn load_audio_file() -> Vec<u8> {
    std::fs::read("assets/sound.wav").unwrap()
}

#[cfg(target_os = "android")]
fn load_audio_file() -> Vec<u8> {
    use std::io::Read;
    let app = APP.lock().unwrap().as_ref().unwrap().clone();
    let asset_manager = app.asset_manager();
    let path = std::ffi::CString::new("sound.wav").unwrap();
    let mut asset = asset_manager.open(path.as_c_str()).unwrap();
    let mut buffer = Vec::new();
    asset.read_to_end(&mut buffer).unwrap();
    buffer
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    APP.lock().unwrap().replace(app);
    run();
}

pub fn run() {
    let event_loop = {
        let mut builder = winit::event_loop::EventLoop::builder();
        #[cfg(target_os = "android")]
        {
            use winit::platform::android::EventLoopBuilderExtAndroid;
            builder.with_android_app(APP.lock().unwrap().as_ref().unwrap().clone());
        }
        builder.build().unwrap()
    };
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
