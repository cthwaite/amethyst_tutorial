use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use pong::{Side, Paddle};


pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle")
            };

            // This is our first attempt at moving the paddles: we take the
            // movement, and scale it by some factor to make the motion seem
            // smooth.
            // In a real game, we would use the time elapsed between frames
            // to determine how far to move the paddle, so that the behavior
            // of the game would not be tied to the game's framerate,
            // but this will do for now. If you run the game now,
            // you'll notice the paddles are able to "fall" off the edges of
            // the game area.
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                transform.translation[1] += scaled_amount;
            }
        }
    }
}
