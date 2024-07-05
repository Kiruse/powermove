use bevy::prelude::*;

use super::GameState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), enter);
  }
}

pub fn enter() {
  debug!("Entering InGame");
}
