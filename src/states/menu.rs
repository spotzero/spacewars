use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Transparent,
    ui::{Anchor, UiText, UiTransform},
    //    window::ScreenDimensions,
    GameData,
    SimpleState,
    SimpleTrans,
    StateData,
    Trans,
};

use crate::resources::*;
use crate::states::*;
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct MenuState;

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.fetch_mut::<Game>().current_state = CurrentState::Menu;
        data.world.fetch_mut::<Time>().set_time_scale(1.);
        data.world.delete_all();
        init_menu(data.world);
        start_text(data.world);
        initialise_camera(data.world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::Space)
                || is_key_down(&event, VirtualKeyCode::Return)
            {
                return SimpleTrans::Switch(Box::new(SpacewarsState));
            }
        }

        // Keep going
        Trans::None
    }
}

fn init_menu(world: &mut World) {
    let bg = world
        .fetch::<AssetManager>()
        .get_render("backgrounds/background-2")
        .unwrap();

    let title = world.fetch::<AssetManager>().get_render("logo").unwrap();

    let controls = world
        .fetch::<AssetManager>()
        .get_render("controls")
        .unwrap();

    let mut bg_transform = Transform::default();
    let scale = ARENA_WIDTH / 1000.0;
    bg_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, -10.0);
    bg_transform.set_scale(Vector3::new(scale, scale, scale));
    world
        .create_entity()
        .with(bg)
        .with(bg_transform)
        .with(Transparent)
        .build();

    let mut title_transform = Transform::default();
    title_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 1.333, 0.0);
    title_transform.set_scale(Vector3::new(1.5, 1.5, 1.));
    world
        .create_entity()
        .with(title)
        .with(title_transform)
        .with(Transparent)
        .build();

    let mut control_transform = Transform::default();
    control_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 4., 0.0);
    control_transform.set_scale(Vector3::new(0.6, 0.6, 1.));
    world
        .create_entity()
        .with(controls)
        .with(control_transform)
        .with(Transparent)
        .build();
}

fn start_text(world: &mut World) {
    let font = world.fetch::<AssetManager>().font().unwrap();
    let pause = UiTransform::new(
        "Press Enter to start".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.0,
        600.,
        50.,
    );

    world
        .create_entity()
        .with(pause)
        .with(UiText::new(
            font.clone(),
            "Press Enter to start".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.,
        ))
        .build();
}
