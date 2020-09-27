use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
};

use crate::pong::{Ball, Pause, BALL_VELOCITY_X, BALL_VELOCITY_Y};

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Pause>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut translations, pause, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        if !pause.paused {
            let delta = time.delta_seconds();
            for (ball, translation) in (&mut balls, &mut translations).join() {
                translation.prepend_translation_x(ball.velocity[0] * delta);
                translation.prepend_translation_y(ball.velocity[1] * delta);

                // Semi-messy way of accelerating the ball if it's been slowed due
                // to a player scoring
                if ball.velocity[0] < BALL_VELOCITY_X {
                    let direction = if ball.velocity[0] > 0. { 1. } else { -1. };
                    ball.velocity[0] = (ball.velocity[0] * direction
                        + delta * BALL_VELOCITY_X * 0.3)
                        .min(BALL_VELOCITY_X)
                        * direction;
                }
                if ball.velocity[1] < BALL_VELOCITY_Y {
                    let direction = if ball.velocity[1] > 0. { 1. } else { -1. };
                    ball.velocity[1] = (ball.velocity[1] * direction
                        + delta * BALL_VELOCITY_Y * 0.3)
                        .min(BALL_VELOCITY_Y)
                        * direction;
                }
            }
        }
    }
}
