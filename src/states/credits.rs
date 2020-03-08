use amethyst::{
    core::timing::Time,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{Anchor, UiText, UiTransform},
    //    window::ScreenDimensions,
    GameData,
    SimpleState,
    SimpleTrans,
    StateData,
    Trans,
};

use crate::resources::*;
use crate::ARENA_WIDTH;

pub struct CreditsState;

impl SimpleState for CreditsState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.fetch_mut::<Game>().current_state = CurrentState::Menu;
        data.world.fetch_mut::<Time>().set_time_scale(1.);
        data.world.delete_all();
        credits_text(data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event)
                || is_key_down(&event, VirtualKeyCode::Escape)
                || is_key_down(&event, VirtualKeyCode::Return)
                || is_key_down(&event, VirtualKeyCode::Space)
            {
                return Trans::Quit;
            }
        }

        // Keep going
        Trans::None
    }
}

fn credits_text(world: &mut World) {
    let credits = [
        "Credits",
        "",
        "Programming, art and graphics, and sound by:",
        "David Pascoe-Deslauriers (@spotzero on github)",
        "",
        "Built with the Amethyst Game Engine",
        "and the Rust programming language.",
        "",
        "",
        "Music Attibutions",
        "",
        "\"Ignition, Set, GO!\" by Bomb Boy is licensed under a",
        "Attribution-Noncommercial-Share Alike 3.0 United States License.",
        "\"Five Nine Seven Eight\" by Virt is licensed under a",
        "Attribution-Noncommercial-Share Alike 3.0 United States License.",
        "\"Endorphemeral\" by Zabutom is licensed under a",
        "Attribution-Noncommercial-Share Alike 3.0 United States License.",
        "\"From the Dunes\" by Synapsis is licensed under a",
        "Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 License.",
        "",
        "",
        "Thanks for playing!",
    ];
    let font = world.fetch::<AssetManager>().font().unwrap();
    let mut offset = 500.;

    for line in credits.iter() {
        let t = UiTransform::new(
            line.to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            offset,
            0.0,
            ARENA_WIDTH,
            35.,
        );

        world
            .create_entity()
            .with(t)
            .with(UiText::new(
                font.clone(),
                line.to_string(),
                [1.0, 1.0, 1.0, 1.0],
                35.,
            ))
            .build();
        offset = offset - 40.;
    }
}
