use butter::{Settings, window};
use butter::winit;
use butter::ButterEngine;

fn main() {
    let engine = ButterEngine::with_settings(Settings {
        window_settings: window::Settings {
            title: "Window".into(),
            ..Default::default()
        }
    });

    winit::ButterRunner::run(engine);
}
