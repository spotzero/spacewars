use amethyst::{
    assets::Loader,
    core::math::Vector3,
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::debug_drawing::DebugLinesComponent,
    renderer::{Camera, Transparent},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    //    window::ScreenDimensions,
    GameData,
    SimpleState,
    SimpleTrans,
    StateData,
    Trans,
};

use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct SpacewarsState;

impl SimpleState for SpacewarsState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.fetch_mut::<Game>().current_state = CurrentState::Playing;
        reset_game(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
                //return SimpleTrans::Switch(Box::new(MenuState));
            }
        }

        Trans::None
    }
}

fn reset_game(world: &mut World) {
    world.delete_all();
    initialise_entities(world);
    initialise_camera(world);
    initialise_ui(world);
}

fn initialise_entities(world: &mut World) {
    let bg_ss = world
        .fetch::<AssetManager>()
        .get_render("backgrounds/background-2")
        .unwrap();
    let gw_ss = world
        .fetch::<AssetManager>()
        .get_render("backgrounds/gravity-well")
        .unwrap();

    let mut bg_transform = Transform::default();
    let scale = ARENA_WIDTH / 1000.0;
    bg_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, -10.0);
    bg_transform.set_scale(Vector3::new(scale, scale, scale));
    world.create_entity().with(bg_ss).with(bg_transform).build();

    let mut gravitywell_transform = Transform::default();
    gravitywell_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, -3.0);
    gravitywell_transform.set_scale(Vector3::new(0.5, 0.5, 1.0));

    world
        .create_entity()
        .with(gw_ss)
        .with(gravitywell_transform)
        .with(Transparent)
        .with(Collidable {
            kind: collidable_types::GRAVITYWELL,
            radius: 20.0,
            ignore: None,
        })
        .with(Movable {
            velocity: Vector3::new(0., 0., 0.),
            angular_velocity: 0.,
            mass: 1000000.,
            apply_physics: false,
        })
        .with(DebugLinesComponent::with_capacity(16))
        .build();
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/UbuntuMono-R.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let mut p1_transform = UiTransform::new(
        "p1-stats".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        20.0,
        0.0,
        0.0,
        300.,
        50.,
    );

    world
        .create_entity()
        .with(p1_transform.clone())
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [1.0, 0.0, 0.0, 1.0],
            50.,
        ))
        .with(StatusUi {
            data: StatusUiKind::Energy,
            player: 1,
        })
        .build();

    p1_transform.local_y -= 50.0;
    world
        .create_entity()
        .with(p1_transform.clone())
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [1.0, 0.0, 0.0, 1.0],
            50.,
        ))
        .with(StatusUi {
            data: StatusUiKind::Shields,
            player: 1,
        })
        .build();

    p1_transform.local_y -= 50.0;
    world
        .create_entity()
        .with(p1_transform.clone())
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [1.0, 0.0, 0.0, 1.0],
            50.,
        ))
        .with(StatusUi {
            data: StatusUiKind::Hull,
            player: 1,
        })
        .build();

    p1_transform.local_y -= 50.0;
    world
        .create_entity()
        .with(p1_transform.clone())
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [1.0, 0.0, 0.0, 1.0],
            50.,
        ))
        .with(StatusUi {
            data: StatusUiKind::Score,
            player: 1,
        })
        .build();

    let mut p2_transform = UiTransform::new(
        "p2-stats".to_string(),
        Anchor::TopRight,
        Anchor::TopRight,
        -20.0,
        0.0,
        0.0,
        300.,
        50.,
    );

    world
        .create_entity()
        .with(p2_transform.clone())
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [0.0, 0.0, 1.0, 1.0],
            50.,
        ))
        .with(StatusUi {
            data: StatusUiKind::Energy,
            player: 2,
        })
        .build();

    p2_transform.local_y -= 50.0;
    world
        .create_entity()
        .with(p2_transform.clone())
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [0.0, 0.0, 1.0, 1.0],
            50.,
        ))
        .with(StatusUi {
            data: StatusUiKind::Shields,
            player: 2,
        })
        .build();

    p2_transform.local_y -= 50.0;
    world
        .create_entity()
        .with(p2_transform.clone())
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [0.0, 0.0, 1.0, 1.0],
            50.,
        ))
        .with(StatusUi {
            data: StatusUiKind::Hull,
            player: 2,
        })
        .build();

    p2_transform.local_y -= 50.0;
    world
        .create_entity()
        .with(p2_transform.clone())
        .with(UiText::new(
            font.clone(),
            "".to_string(),
            [0.0, 0.0, 1.0, 1.0],
            50.,
        ))
        .with(StatusUi {
            data: StatusUiKind::Score,
            player: 2,
        })
        .build();

    let mut sop = StatusOfPlayers::default();
    sop.players.insert(
        1,
        StatusOfPlayer {
            id: 1,
            energy: 0.,
            shields: 0.,
            hull: 0.,
            dead: true,
            lives: 5,
            respawn: 0.,
        },
    );
    sop.players.insert(
        2,
        StatusOfPlayer {
            id: 2,
            energy: 0.,
            shields: 0.,
            hull: 0.,
            dead: true,
            lives: 5,
            respawn: 0.,
        },
    );
    world.insert(sop);
}
