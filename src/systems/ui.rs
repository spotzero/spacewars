use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    ui::{UiText, UiTransform},
};

use crate::resources::StatusUi;

#[derive(SystemDesc)]
pub struct StatusUiSystem;

impl<'s> System<'s> for StatusUiSystem {
    type SystemData = (
        ReadStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, StatusUi>,
    );

    fn run(
        &mut self,
        (
            mut ui_transforms,
            mut ui_texts,
            status_ui,
        ): Self::SystemData,
    ) {
    }
}
