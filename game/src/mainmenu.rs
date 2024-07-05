use bevy::prelude::*;

use super::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), enter);
  }
}

pub fn enter() {
  println!("Entering main menu");
}
