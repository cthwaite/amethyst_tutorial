extern crate amethyst;

use amethyst::prelude::*;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, Pipeline, PosTex,
                         RenderBundle, Stage, VirtualKeyCode};


// Core game struct.
pub struct Pong;

impl<'a, 'b> State<GameData<'a, 'b>> for Pong {
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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&path);

    // The important thing to know right now is that this renders a black
    // background. If you want a different color you can tweak the RGBA values
    // inside the .clear_target method. Values range from 0.0 to 1.0, so to
    // get that cool green color we started with back, for instance, you can
    // try [0.00196, 0.23726, 0.21765, 1.0].
    let pipe = Pipeline::build().with_stage(Stage::with_backbuffer()
                                            .clear_target([0.722, 0.114, 0.325, 1.0], 1.0)
                                            .with_pass(DrawFlat::<PosTex>::new()));

    // Here we're creating a new RenderBundle, adding the Pipeline we created,
    // along with our config, and building.
    // There is also a helper function with_basic_renderer on GameDataBuilder
    // that you can use to create your Pipeline and RenderBundle, that performs
    // most of the actions above. In the full pong example in the Amethyst
    // repository, that function is used instead.
    let game_data = GameDataBuilder::default()
                        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    // [Application] binds the OS event loop, state machines, timers and other
    // core components in a central place.
    let mut game = Application::new("./", Pong, game_data)?;

    // Then we call .run() on game which begins the gameloop.
    // The game will continue to run until our State returns Trans::Quit, or
    // when all states have been popped off the state machine's stack.
    game.run();
    Ok(())
}
