use amethyst::{
    core::timing::Time,
    ecs::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{Anchor, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

use crate::resources::*;

pub struct PauseState;

struct PauseText {
    text: Entity,
}

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.fetch_mut::<Game>().current_state = CurrentState::Pause;
        data.world.fetch_mut::<Time>().set_time_scale(0.);
        let pause_text = PauseText {
            text: pause_text(data.world),
        };
        data.world.insert(pause_text);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let text = data.world.remove::<PauseText>().unwrap();
        let _ = data.world.entities().delete(text.text);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }

            if is_key_down(&event, VirtualKeyCode::Back) {
                data.world.fetch_mut::<Game>().game_state = GameState::Tie;
                data.world.fetch_mut::<Game>().end_time =
                    data.world.fetch::<Time>().absolute_time_seconds();
                return Trans::Pop;
            }
        }

        Trans::None
    }
}

fn pause_text(world: &mut World) -> Entity {
    let font = world.fetch::<AssetManager>().font().unwrap();
    let pause = UiTransform::new(
        "Paused".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        200.0,
        0.0,
        300.,
        50.,
    );

    return world
        .create_entity()
        .with(pause)
        .with(UiText::new(
            font.clone(),
            "Paused".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.,
        ))
        .build();
}
