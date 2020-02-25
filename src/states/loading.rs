use amethyst::{
    assets::Loader,
    audio::{AudioSink, Mp3Format},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::resources::*;
use crate::states::*;

pub struct LoadingState;

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        initialise_resources(data.world);
        load_music(data.world);
        load_assets(data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let asset_manager = data.world.fetch::<AssetManager>();
        if asset_manager.progress.is_complete() {
            // Loaded.
            return SimpleTrans::Switch(Box::new(SpacewarsState));
        }
        Trans::None
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
        }
        Trans::None
    }
}

fn load_music(world: &mut World) {
    //let loader = world.read_resource::<Loader>();
    let mut tracks = vec![
        "music/Virt_-_07_-_Five_Nine_Seven_Eight.mp3",
        "music/Synapsis_-_08_-_From_the_Dunes.mp3",
        "music/Bomb_Boy_-_06_-_Ignition_Set_GO.mp3",
        "music/Zabutom_-_17_-_Endorphemeral.mp3",
    ];
    tracks.shuffle(&mut thread_rng());
    world.write_resource::<AudioSink>().set_volume(0.25);
    world.insert(Music {
        music: tracks
            .iter()
            .map(|file| {
                world.read_resource::<Loader>().load(
                    file.to_string(),
                    Mp3Format,
                    (),
                    &world.read_resource(),
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .cycle(),
    });
}

fn load_assets(world: &mut World) {
    let mut asset_manager = world.fetch_mut::<AssetManager>();
    asset_manager.insert(world, "font/UbuntuMono-R.ttf", AssetKind::Font);

    asset_manager.insert(world, "backgrounds/background-2", AssetKind::Sprite);
    asset_manager.insert(world, "backgrounds/gravity-well", AssetKind::Sprite);
    asset_manager.insert(world, "ships/ship-001", AssetKind::Sprite);
    asset_manager.insert(world, "ships/ship-002", AssetKind::Sprite);
    asset_manager.insert(world, "ships/shields", AssetKind::Sprite);
    asset_manager.insert(world, "particles/particle0", AssetKind::Sprite);
    asset_manager.insert(world, "particles/debris", AssetKind::Sprite);
    asset_manager.insert(world, "weapons/missle-001", AssetKind::Sprite);

    asset_manager.insert(world, "clank-hull", AssetKind::Sound);
    asset_manager.insert(world, "clank-shield", AssetKind::Sound);
    asset_manager.insert(world, "engine-pulse", AssetKind::Sound);
    asset_manager.insert(world, "torpedo", AssetKind::Sound);
    asset_manager.insert(world, "explosion-player", AssetKind::Sound);
    asset_manager.insert(world, "explosion-torpedo", AssetKind::Sound);
    asset_manager.insert(world, "railgun", AssetKind::Sound);
}

fn initialise_resources(world: &mut World) {
    world.insert(Game {
        current_state: CurrentState::Loading,
    });
    world.insert(StatusOfPlayers::default());
    world.insert(AssetManager::default());
    world.insert(CollisionEvents::default());
    world.insert(DamageEvents::default());
    world.insert(AudioEvents::default());
    world.insert(AudioState::default());
}
