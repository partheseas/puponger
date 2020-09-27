use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::pong::{Ball, Pause};

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Pause>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut translations, pause, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        if !pause.paused {
            for (ball, translation) in (&balls, &mut translations).join() {
                translation.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
                translation.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
            }
        }
    }
}
