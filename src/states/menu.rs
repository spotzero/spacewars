use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

use crate::resources::*;
use crate::states::*;

pub struct MenuState;

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.fetch_mut::<Game>().current_state = CurrentState::Menu;
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

            if is_key_down(&event, VirtualKeyCode::Space) {
                return SimpleTrans::Switch(Box::new(SpacewarsState));
            }
        }

        //for world.fetch::<StatusOfPlayers>()

        // Keep going
        Trans::None
    }
}
