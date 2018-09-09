// Core game struct.
pub struct Pong;

use amethyst::prelude::*;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender
};


const ARENA_WIDTH: f32 = 100.0;
const ARENA_HEIGHT: f32 = 100.0;


// We create an entity that will carry our camera, with an orthographic
// projection of the size of our arena (as we want it to cover it all).
// Ignore the GlobalTransform for now, we'll deal with it in more details
// later on.
// Note that as the origin of our camera is in the bottom left corner, we set
// ARENA_HEIGHT as the top and 0.0 as the bottom.
fn initialize_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(0.0, ARENA_WIDTH, ARENA_HEIGHT, 0.0)))
        .with(GlobalTransform(Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()))
        .build();
}


const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;
const SPRITESHEET_SIZE: (f32, f32) = (8.0, 16.0);

#[derive(PartialEq, Eq)]
pub enum Side {
    Left, Right
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32
}

impl Paddle {
    fn new(side: Side) -> Self {
        Paddle {
            side,
            width: 1.0,
            height: 1.0
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_paddles(world: &mut World, spritesheet: TextureHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_transform.translation = Vector3::new(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.translation = Vector3::new(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite = Sprite {
        left: 0.0,
        right: PADDLE_WIDTH,
        top: 0.0,
        bottom: PADDLE_HEIGHT,
    };


    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
            .expect("Failed to add sprite render on left paddle")
        .with(Paddle::new(Side::Left))
        .with(GlobalTransform::default())
        .with(left_transform)
        .build();
    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
            .expect("Failed to add sprite render on right paddle")
        .with(Paddle::new(Side::Right))
        .with(GlobalTransform::default())
        .with(right_transform)
        .build();
}


impl<'a, 'b> State<GameData<'a, 'b>> for Pong {

    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let spritesheet = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load("texture/pong_spritesheet.png",
                        PngFormat,
                        Default::default(),
                        (),
                        &texture_storage)
        };
        world.register::<Paddle>();
        initialize_paddles(world, spritesheet);
        initialize_camera(world);
    }


    // The handle_event method is executed for every event before updating, and
    // it's used to react to events. It returns a Trans, which is an enum of
    // state machine transitions. In this case, we're watching for the Escape
    // keycode, and the CloseRequested event from the Window. If we receive it,
    // we'll return Trans::Quit which will be used to clean up the State and
    // close the application.
    // All other keyboard input is ignored for now.
    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

