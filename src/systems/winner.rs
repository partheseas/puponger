use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, ReadExpect, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
};

use crate::pong::{Ball, Pong, ScoreBoard, ScoreText, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        // WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        // ReadExpect<'s, ScoreText>,
    );

    fn run(
        &mut self,
        // (mut balls, mut transforms, mut ui_text, mut scores, score_text): Self::SystemData,
        (mut balls, mut transforms, mut scores): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let ball_x = transform.translation().x;

            let did_score = if ball_x <= ball.radius {
                // Right player scored on the left side.
                // We top the score at 999 to avoid text overlap.
                println!("Player 2 Scores!");
                scores.score_right = (scores.score_right + 1).min(999);
                // if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                //     text.text = scores.score_right.to_string();
                // }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                // We top the score at 999 to avoid text overlap.
                println!("Player 1 Scores!");
                scores.score_left = (scores.score_left + 1).min(999);
                // if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                //     text.text = scores.score_left.to_string();
                // }
                true
            } else {
                false
            };

            if did_score {
                ball.velocity[0] = -ball.velocity[0]; // Reverse Direction
                transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position
                transform.set_translation_y(ARENA_HEIGHT / 2.0); // Reset Position

                // Print the scoreboard.
                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right
                );
            }
        }
    }
}
