extern crate component_derive;

mod lib;

use lib::ecs::{Component, System};
use component_derive::Component;
fn main() {

    let mut system = System::default();
    let _player = system.create_entity()
        .set_id(5)
        .set_name("criosage")
        .add_component(Position::new(3, 5))
        .build()
        .expect("Failed to create player");
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}