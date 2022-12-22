use butter::window;
use butter::winit;
use butter::ButterEngine;

fn main() {
    let engine = ButterEngine;
    let window_settings = window::Settings {
        title: "Window",
        size: window::Size {
            width: 800,
            height: 600,
        },
        ..Default::default()
    };

    winit::ButterRunner::run(&engine, &window_settings);
}
