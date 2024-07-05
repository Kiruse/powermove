use bevy::{log::LogPlugin, prelude::*};

pub mod mainmenu;
pub mod ingame;
pub mod loading;
pub mod procedural;
pub mod world;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
  #[default]
  Initial,
  Loading,
  MainMenu,
  InGame,
}

fn main() {
  App::new()
    .init_state::<GameState>()
    .add_plugins(DefaultPlugins.set(LogPlugin {
      filter: "info,abysine=debug".into(),
      level: bevy::log::Level::DEBUG,
      ..default()
    }))
    .add_plugins((
      crate::mainmenu::MainMenuPlugin,
      crate::loading::LoadingPlugin,
      crate::ingame::InGamePlugin,
    ))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
  mut next_state: ResMut<NextState<GameState>>
) {
  debug!("Setup complete");
  next_state.set(GameState::InGame);
}
