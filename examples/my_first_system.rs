extern crate amethyst;

use amethyst::ecs::{DispatcherBuilder, System, World};

struct MyFirstSystem;

impl <'a> System<'a> for MyFirstSystem {
    type SystemData = ();
    fn run(&mut self, _data: Self::SystemData) {
        println!("Hello from MyFirstSystem!");
    }
}

fn main() {
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
                            .with(MyFirstSystem, "my_first_system", &[])
                            .build();
    dispatcher.setup(&mut world.res);
    dispatcher.dispatch(&mut world.res);
}
