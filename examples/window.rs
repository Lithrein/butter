use butter::ButterEngineBuilder;

fn main() {
    let engine = ButterEngineBuilder::new()
        .with_window_title("Window")
        .build();
    butter::winit::ButterRunner::run(engine);
}
