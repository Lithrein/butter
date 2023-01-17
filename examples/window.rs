use butter::ecs::commands::CommandQueue;
use butter::ecs::query::Query;
use butter::ButterEngineBuilder;

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

fn init(command_queue: &mut CommandQueue) {
    command_queue.insert((Player("John Doe"),));
    command_queue.insert((Player("Jack Doe"),));
}

fn hello_world(_: &mut CommandQueue) {
    println!("hello world");
}

fn hello_player(_: &mut CommandQueue, players: &Query<(&Player,)>) {
    for (player,) in players.iter() {
        println!("hello {}", player.0);
    }
}
