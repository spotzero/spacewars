use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteExpect},
    ecs::{world::EntitiesRes, Entities, Entity, LazyUpdate, ReadExpect},
    input::{InputHandler, StringBindings},
    renderer::debug_drawing::DebugLinesComponent,
    renderer::transparent::Transparent,
};

use crate::{components::*, resources::*};

#[derive(SystemDesc)]
pub struct AudioManagerSystem;

impl<'s> System<'s> for AudioManagerSystem {
    type SystemData = (
        WriteExpect<'s, AudioEvents>,
        WriteExpect<'s, AudioState>,
        ReadExpect<'s, AssetManager>,
        Read<'s, AssetStorage<Source>>,
        Read<'s, Output>,
    );

    fn run(
        &mut self,
        (mut audio_events, mut audio_state, asset_manager, storage, audio_output): Self::SystemData,
    ) {
        for i in 0..audio_events.events.len() {
            match audio_events.events[i] {
                AudioEvent::Engine { player, state } => {
                    let cur = audio_state
                        .engines
                        .get(&player)
                        .or(Some(&false))
                        .expect("Engine audio issue");
                    if !cur && state {
                        asset_manager.play_wav("engine-pulse", &storage, &audio_output);
                    }
                    audio_state.engines.insert(player, state);
                }
                AudioEvent::HullHit => {
                    asset_manager.play_wav("clank-hull", &storage, &audio_output)
                }
                AudioEvent::ShieldHit => {
                    asset_manager.play_wav("clank-shield", &storage, &audio_output)
                }
                AudioEvent::ExplosionPlayer => {
                    asset_manager.play_wav("explosion-player", &storage, &audio_output)
                }
                AudioEvent::ExplosionTorpedo => {
                    asset_manager.play_wav("explosion-torpedo", &storage, &audio_output)
                }
                AudioEvent::Railgun => asset_manager.play_wav("railgun", &storage, &audio_output),
                AudioEvent::Torpedo => asset_manager.play_wav("torpedo", &storage, &audio_output),
            }
        }
        audio_events.events.clear();
    }
}
