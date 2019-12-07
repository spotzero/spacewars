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
        builder.add(ParticleSystem, "particle_system", &[]);
        builder.add(EngineParticleSystem, "engine_particle_system", &[]);
        builder.add(RechargeSystem, "recharge_system", &[]);
        builder.add(CollisionSystem, "collision_system", &[]);
        builder.add(DebugCollisionSystem, "debug_collision_system", &["collision_system"]);
        builder.add(FireTorpedoSystem, "fire_torpedo_system", &["input_system"]);
        builder.add(ExplodeTorpedoSystem, "explode_torpedo_system", &[]);
        builder.add(ExplosionSystem, "explosion_system", &[]);
        builder.add(TorpedoCollisionResponseSystem, "torpedo_collision_response_system", &["collision_system"]);
        builder.add(ExplosionCollisionResponseSystem, "explosion_collision_response_system", &["collision_system", "explosion_system"]);
        builder.add(DamageSystem, "damage_system", &["explosion_collision_response_system"]);
        builder.add(PlayerDeathSystem, "player_death_system", &["damage_system"]);
        builder.add(PlayerRespawnSystem, "player_respawn_system", &[]);
        builder.add(StatusUpdateSystem, "status_update_system", &[]);
        builder.add(StatusUiSystem, "status_ui_system", &["status_update_system"]);
        Ok(())
    }
}
