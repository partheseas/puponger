use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write},
    input::{InputHandler, StringBindings},
};

use crate::pong::Pause;

#[derive(SystemDesc)]
pub struct PauseSystem;

impl<'s> System<'s> for PauseSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>, Write<'s, Pause>);

    fn run(&mut self, (input, mut pause): Self::SystemData) {
        if let Some(true) = input.action_is_down("pause") {
            if !pause.changed_last_frame {
                pause.changed_last_frame = true;
                pause.paused = !pause.paused;
            }
        } else {
            pause.changed_last_frame = false;
        };
    }
}
