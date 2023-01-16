use butter::{ecs::query::Query, ButterEngineBuilder};

pub struct Player(&'static str);

fn main() {
    let engine = ButterEngineBuilder::new()
        .with_window_title("Window")
        .with_init_system(init)
        .with_system(hello_world)
        .with_system(hello_player)
        .build();
    butter::winit::ButterRunner::run(engine);
}

fn init() {
    println!("This system will run once");
}

fn hello_world() {
    println!("hello world");
}

fn hello_player(players: &Query<(&Player,)>) {
    for (player,) in players.iter() {
        println!("hello {}", player.0);
    }
}
