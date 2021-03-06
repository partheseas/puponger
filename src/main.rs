mod audio;
mod pong;
mod systems;

use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use audio::Music;
use pong::Pong;
use systems::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("bindings.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config_path)?,
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.1, 1.0, 0.6, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with(PauseSystem, "pause_system", &[])
        .with(PaddleSystem, "paddle_system", &["input_system"])
        .with(BallSystem, "ball_system", &[])
        .with(
            BounceSystem,
            "bounce_system",
            &["paddle_system", "ball_system"],
        )
        .with(WinnerSystem, "winner_system", &["ball_system"])
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        );

    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}
