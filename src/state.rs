use amethyst::{
    assets::Loader,
    ecs::Entity,
    core::math::Point3,
    core::math::Vector3,
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, Transparent},
    renderer::debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
    renderer::palette::Srgba,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
//    window::ScreenDimensions,
};

use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::components::*;
use crate::resources::*;

pub struct SpacewarsState;

impl SimpleState for SpacewarsState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let mut sprite_sheet_manager = SpriteSheetManager::default();
        sprite_sheet_manager.insert(&mut world, "backgrounds/background-2");
        sprite_sheet_manager.insert(&mut world, "backgrounds/gravity-well");
        sprite_sheet_manager.insert(&mut world, "ships/ship-001");
        sprite_sheet_manager.insert(&mut world, "particles/particle0");
        sprite_sheet_manager.insert(&mut world, "particles/debris");
        sprite_sheet_manager.insert(&mut world, "weapons/missle-001");

        let mut bg_transform = Transform::default();
        bg_transform.set_translation_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0, -10.0);

        world
            .create_entity()
            .with(sprite_sheet_manager.get_render("backgrounds/background-2").unwrap())
            .with(bg_transform)
            .build();

        let mut gravitywell_transform = Transform::default();
        gravitywell_transform.set_translation_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0, -9.0);
        gravitywell_transform.set_scale(Vector3::new(0.5,0.5,1.0));

        world
            .create_entity()
            .with(sprite_sheet_manager.get_render("backgrounds/gravity-well").unwrap())
            .with(gravitywell_transform)
            .with(Transparent)
            .with(Collidable {
                kind: collidable_types::GRAVITYWELL,
                radius: 25.0,
            })
            .build();

        // Place the camera
        initialise_camera(world);
        initialise_collision(world);
        initialise_ui(world);
        world.insert(sprite_sheet_manager);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        // Keep going
        Trans::None
    }
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

fn initialise_collision(world: &mut World) {
    world.insert(CollisionEvents::default());
    world.insert(DamageEvents::default());

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
        .with(StatusUi{
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
        .with(StatusUi{
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
        .with(StatusUi{
            data: StatusUiKind::Hull,
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
        .with(StatusUi{
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
        .with(StatusUi{
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
        .with(StatusUi{
            data: StatusUiKind::Hull,
            player: 2,
        })
        .build();

    let mut sop = StatusOfPlayers::default();
    sop.players.insert(1, StatusOfPlayer{
        id: 1,
        energy: 0.,
        shields: 0.,
        hull: 0.,
        dead: true,
        lives: 5,
        respawn: 0.,
    });
    sop.players.insert(2, StatusOfPlayer{
        id: 2,
        energy: 0.,
        shields: 0.,
        hull: 0.,
        dead: true,
        lives: 5,
        respawn: 0.,
    });
    world.insert(sop);
}
