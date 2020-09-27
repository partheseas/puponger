use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Pause, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Pause>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, paddles, input, pause, time): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if !pause.paused {
                if let Some(mv_amount) = movement {
                    let scaled_amount = time.delta_seconds() * 80.0 * mv_amount as f32;
                    let paddle_y = (scaled_amount + transform.translation().y)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(PADDLE_HEIGHT * 0.5);
                    transform.set_translation_y(paddle_y);
                }
            }
        }
    }
}
