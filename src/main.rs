extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
    ui::{RenderUi, UiBundle},
};

mod bundle;
mod components;
mod state;
mod systems;
mod resources;

use crate::bundle::SpacewarsBundle;

const ARENA_HEIGHT: f32 = 1000.0;
const ARENA_WIDTH: f32 = 1000.0;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("config/display_config.ron");
    let key_bindings_path = resources.join("config/bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(SpacewarsBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config))
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(resources, state::SpacewarsState, game_data)?;
    game.run();

    Ok(())
}
