use bevy::ecs::world::Command;
use bevy::prelude::*;
use bevy_sprite3d::*;
use lib::directions::Direction;

use crate::loaders::behaviors::character::{CharAnimState, CharacterBehavior, CharacterBehaviorLoader};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_asset::<CharacterBehavior>()
      .init_asset_loader::<CharacterBehaviorLoader>()
      .add_systems(Update, (observe_asset_events, animate));
  }
}

/// Character component storing general behavior state. Should only be spawned by the
/// [`SpawnCharacter`] command.
#[derive(Component, Debug, Default)]
pub struct Character {
  pub behavior: Handle<CharacterBehavior>,
  pub anim_state: CharAnimState,
  pub direction: Direction,
}

#[derive(Component)]
struct CharacterUninitialized {
  transform: Transform,
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct CharacterAnimationTimer(pub Timer);

pub struct SpawnCharacter {
  asset_path: String,
  transform: Transform,
}

impl SpawnCharacter {
  pub fn new(asset_path: impl Into<String>) -> Self {
    SpawnCharacter {
      asset_path: asset_path.into(),
      transform: Transform::default(),
    }
  }

  pub fn with_transform(mut self, transform: Transform) -> Self {
    self.transform = transform;
    self
  }
}

impl Command for SpawnCharacter {
  fn apply(self, world: &mut World) {
    let asset_srv = world.get_resource::<AssetServer>().unwrap();
    world.spawn((
      Character {
        behavior: asset_srv.load(&self.asset_path),
        ..Default::default()
      },
      CharacterUninitialized {
        transform: self.transform,
      },
    ));
  }
}

fn observe_asset_events(
  reader: EventReader<AssetEvent<CharacterBehavior>>,
  behaviors: Res<Assets<CharacterBehavior>>,
  mut commands: Commands,
  mut query: Query<(Entity, &Character, &CharacterUninitialized)>,
  mut sprite3d_params: Sprite3dParams,
) {
  if !has_finished_loading_event(reader) { return }

  for (entity, char, uninit) in query.iter_mut() {
    if let Some(behavior) = behaviors.get(&char.behavior) {
      commands.entity(entity)
        .remove::<CharacterUninitialized>()
        .insert((
          Sprite3d {
            image: behavior.sprite.clone(),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            double_sided: true,
            transform: uninit.transform,
            ..Default::default()
          }.bundle_with_atlas(
            &mut sprite3d_params,
            TextureAtlas {
              layout: behavior.atlas_layout.clone(),
              index: 0,
            },
          ),
          CharacterAnimationTimer(Timer::new(behavior.frame_duration(CharAnimState::Idle), TimerMode::Repeating)),
        ));
    }
  }
}

fn animate(
  time: Res<Time>,
  behaviors: Res<Assets<CharacterBehavior>>,
  mut query: Query<(&Character, &mut TextureAtlas, &mut CharacterAnimationTimer)>,
) {
  for (char, mut atlas, mut timer) in query.iter_mut() {
    if let Some(behavior) = behaviors.get(&char.behavior) {
      timer.tick(time.delta());
      if timer.just_finished() {
        atlas.index = behavior.next_frame(atlas.index, char.anim_state, char.direction);
        timer.0.set_duration(behavior.frame_duration(char.anim_state));
      }
    }
  }
}

fn has_finished_loading_event<A: Asset>(mut reader: EventReader<AssetEvent<A>>) -> bool {
  for event in reader.read() {
    match event {
      AssetEvent::LoadedWithDependencies { .. } => return true,
      _ => (),
    }
  }
  return false;
}
