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

    let pipe = Pipeline::build().with_stage(Stage::with_backbuffer()
                                            .clear_target([0.722, 0.114, 0.325, 1.0], 1.0)
                                            .with_pass(DrawFlat::<PosTex>::new())
                                            );
    let game_data = GameDataBuilder::default()
                        .with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::new("./", Pong, game_data)?;
    game.run();
    Ok(())
}
