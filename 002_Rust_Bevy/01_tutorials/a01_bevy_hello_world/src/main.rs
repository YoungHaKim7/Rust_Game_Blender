use bevy::prelude::*;

fn hello_world_system() {
    println!("Hello world Bevy Rust Game Engine");
}

fn main() {
    App::new().add_systems(Update, hello_world_system).run();
}
