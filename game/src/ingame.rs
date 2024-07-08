use bevy::prelude::*;

use super::GameState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), enter);
  }
}

pub fn enter(
  mut cmd: Commands,
  data: Res<crate::LaunchData>,
) {
  debug!("Entering InGame");

  cmd.spawn(Camera3dBundle {
    transform: Transform::from_translation(Vec3::new(4.0, 2.0, 10.0))
      .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ..Default::default()
  });

  cmd.spawn(PbrBundle {
    mesh: data.world.islands[0].mesh.clone(),
    material: data.world.islands[0].mat.clone(),
    ..Default::default()
  });

  cmd.spawn(PointLightBundle {
    transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
    ..Default::default()
  });
}
