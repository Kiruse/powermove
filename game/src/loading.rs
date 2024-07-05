use bevy::prelude::*;

use super::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::Loading), enter);
  }
}

pub fn enter() {
  println!("Entering loading screen");
}

