mod rex;

use crate::rex::entity::Entity;

fn main() {
    let mut player: Entity = Entity::default();
    player.set_id(01);
    player.set_name("criosage");
    player.add_component::<Transform>();
    player.add_component::<Renderer>();
    let transform = player.get_component_mut::<Transform>().unwrap();
    transform.x += 1;
    dbg!(transform);
}

#[derive(Default, Debug)]
struct Transform {
    x: isize,
    y: isize,
}

#[derive(Default)]
struct Renderer {
    r: u8,
    g: u8,
    b: u8,
}
