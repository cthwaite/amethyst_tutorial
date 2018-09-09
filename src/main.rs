extern crate amethyst;
mod pong;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex,
                         RenderBundle, Stage};

use pong::Pong;


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
                        .with_bundle(TransformBundle::new())?
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
