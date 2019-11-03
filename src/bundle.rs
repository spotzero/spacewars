use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

use crate::systems::*;

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct SpacewarsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SpacewarsBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(ShipInputSystem, "shipinput_system", &["input_system"]);
        builder.add(PhysicsSystem, "physics_system", &[]);
        builder.add(ShipSystem, "ship_system", &[]);
        Ok(())
    }
}