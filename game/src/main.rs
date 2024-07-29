use bevy::{log::LogPlugin, prelude::*};
use bevy_obj::ObjPlugin;
use bevy_sprite3d::Sprite3dPlugin;

mod mainmenu;
mod ingame;
mod loaders;
mod loading;
mod character;
mod world;

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
    // default plugins
    .add_plugins((
      DefaultPlugins
      .set(LogPlugin {
        filter: "info,abysine=debug".into(),
        level: bevy::log::Level::DEBUG,
        ..default()
      })
      .set(ImagePlugin::default_nearest()),
    ))
    // 3rd party plugins
    .add_plugins((
      ObjPlugin,
      Sprite3dPlugin,
    ))
    // project plugins
    .add_plugins((
      crate::mainmenu::MainMenuPlugin,
      crate::loading::LoadingPlugin,
      crate::ingame::InGamePlugin,
      crate::character::CharacterPlugin,
    ))
    .init_state::<GameState>()
    .init_resource::<LaunchData>()
    .add_systems(Startup, setup)
    .run();
}

fn setup(
  assets: Res<AssetServer>,
  mut launchres: ResMut<LaunchData>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  launchres.world.islands.push(world::Island {
    mesh: assets.load("models/test-island.obj"),
    mat: assets.add(StandardMaterial {
      base_color: Color::srgb(0.8, 0.7, 0.6),
      ..Default::default()
    }),
  });
  debug!("Setup complete");
  next_state.set(GameState::InGame);
}
