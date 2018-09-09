extern crate amethyst;

use amethyst::ecs::{DispatcherBuilder, Read, Resources, System, World, Write};
use amethyst::shrev::{EventChannel, ReaderId};

#[derive(Debug)]
enum MyEvent {
    A,
    B
}

struct ProdSystem;

impl<'a> System<'a> for ProdSystem {
    type SystemData = Write<'a, EventChannel<MyEvent>>;

    fn run(&mut self, my_event_channel: Self::SystemData) {
        my_event_channel.single_write(MyEvent::A);
    }
}

#[derive(Default)]
struct RecvSystem {
    reader: Option<ReaderId<MyEvent>>
}


impl<'a> System<'a> for RecvSystem {
    type SystemData = Read<'a, EventChannel<MyEvent>>;
    fn setup(&mut self, res: &mut Resources) {
        // Ensures that resources implementing `Default` present in your
        // `SystemData` are added to `Resources`.
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<MyEvent>>().register_reader());
    }

    fn run(&mut self, my_event_channel: Self::SystemData) {
        for event in my_event_channel.read(self.reader.as_mut().unwrap()) {
            println!("Received event value of {:?}", event);
        }
    }
}


fn main() {
    let mut world = World::new();
    world.add_resource(EventChannel::<MyEvent>::new());
    let mut dispatcher = DispatcherBuilder::new()
                            .with(ProdSystem, "prod_system", &[])
                            .with(RecvSystem::default(), "recv_system", &[])
                            .build();
    dispatcher.setup(&mut world.res);
    dispatcher.dispatch(&mut world.res);

}
