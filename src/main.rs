extern crate amethyst;

use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod bundle;
mod components;
mod resources;
mod states;
mod systems;

use crate::bundle::SpacewarsBundle;

const ARENA_HEIGHT: f32 = 960.0;
const ARENA_WIDTH: f32 = 1280.0;
const DEBUG_MODE: bool = false;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources/");
    let display_config = resources.join("config/display_config.ron");
    let key_bindings_path = resources.join("config/bindings.ron");

    let mut rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());

    if DEBUG_MODE {
        rendering_bundle = rendering_bundle.with_plugin(RenderDebugLines::default());
    }

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_bundle(SpacewarsBundle)?
        .with_bundle(rendering_bundle)?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut resources::Music| music.music.next()),
            "dj_system",
            &[],
        );

    let mut game = Application::new(resources, states::LoadingState, game_data)?;
    game.run();

    Ok(())
}
