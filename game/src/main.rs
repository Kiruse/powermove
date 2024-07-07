use bevy::{log::LogPlugin, prelude::*};
use bevy_obj::ObjPlugin;

pub mod mainmenu;
pub mod ingame;
pub mod loading;
pub mod world;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
  #[default]
  Initial,
  Loading,
  MainMenu,
  InGame,
}

#[derive(Resource, Default, Debug)]
pub struct LaunchData {
  pub world: world::World,
}

fn main() {
  App::new()
    .init_state::<GameState>()
    .add_plugins((
      DefaultPlugins.set(LogPlugin {
        filter: "info,abysine=debug".into(),
        level: bevy::log::Level::DEBUG,
        ..default()
      }),
      ObjPlugin,
    ))
    .add_plugins((
      crate::mainmenu::MainMenuPlugin,
      crate::loading::LoadingPlugin,
      crate::ingame::InGamePlugin,
    ))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
  assets: Res<AssetServer>,
  mut launchres: ResMut<LaunchData>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  launchres.world.islands.push(world::Island {
    mesh: assets.load("models/island.obj"),
  });
  debug!("Setup complete");
  next_state.set(GameState::InGame);
}
