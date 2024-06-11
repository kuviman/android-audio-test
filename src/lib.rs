use geng::prelude::*;

struct Test {
    sound: geng::Sound,
}

impl Test {
    async fn new(geng: &Geng) -> Self {
        Self {
            sound: geng.asset_manager().load("./sound.wav").await.unwrap(),
        }
    }
}

impl geng::State for Test {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
    }
    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::TouchStart { .. } = event {
            self.sound.play();
        }
    }
}

#[no_mangle]
fn android_main(app: android::App) {
    android::init(app);

    Geng::run("audio test", |geng| async move {
        geng.run_state(Test::new(&geng).await).await;
    })
}
