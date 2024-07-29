use std::time::Duration;

use bevy::asset::{AsyncReadExt, LoadedAsset};
use bevy::asset::{AssetLoader, io::Reader, LoadContext};
use bevy::prelude::*;
use thiserror::Error;

type BehaviorConfig = lib::behaviors::Behavior;

#[derive(Asset, Debug, TypePath)]
pub struct CharacterBehavior {
  pub config: lib::behaviors::CharacterBehavior,
  pub sprite: Handle<Image>,
  pub atlas_layout: Handle<TextureAtlasLayout>,
}

impl CharacterBehavior {
  pub fn next_frame(&self, current_frame: usize, anim_state: CharAnimState, direction: lib::directions::Direction) -> usize {
    let (min, max) = self.frame_minmax(anim_state, direction);
    if current_frame < min || current_frame > max {
      min
    } else {
      current_frame + 1
    }
  }

  pub fn frame_duration(&self, anim_state: CharAnimState) -> Duration {
    use CharAnimState::*;

    let millis = match anim_state {
      Idle => self.config.anims.idle.dt,
      Walk => self.config.anims.walk.map(|walk| walk.dt.into()).unwrap_or(125),
    };
    Duration::from_millis(millis.into())
  }

  pub fn frame_minmax(&self, anim_state: CharAnimState, direction: lib::directions::Direction) -> (usize, usize) {
    use CharAnimState::*;

    let anims = &self.config.anims;
    match anim_state {
      Idle => self.idle_frame_minmax(),
      Walk =>
        anims.walk.map(|walk| {
          let offset: usize = (walk.offset + (walk.stride * u16::from(direction.index()))).into();
          (offset, offset + walk.stride as usize - 1)
        })
        .unwrap_or(self.idle_frame_minmax()),
    }
  }

  fn idle_frame_minmax(&self) -> (usize, usize) {
    let idle = &self.config.anims.idle;
    (usize::from(idle.offset), usize::from(idle.offset + idle.stride - 1))
  }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum CharAnimState {
  #[default]
  Idle,
  Walk,
}

#[derive(Default)]
pub struct CharacterBehaviorLoader;

impl AssetLoader for CharacterBehaviorLoader {
  type Asset = CharacterBehavior;
  type Settings = ();
  type Error = BehaviorLoaderError;

  async fn load<'a>(
    &'a self,
    reader: &'a mut Reader<'_>,
    _settings: &'a Self::Settings,
    load_context: &'a mut LoadContext<'_>,
  ) -> Result<Self::Asset, Self::Error> {
    let config = load_behavior(reader).await?;
    match config {
      BehaviorConfig::Character(config) => {
        let sprite_asset: LoadedAsset<Image> = load_context
          .loader()
          .direct()
          .load(format!("sprites/{}", config.sprite))
          .await?;
        let sprite = load_context
          .add_loaded_labeled_asset("texture", sprite_asset);

        let atlas_layout_asset = TextureAtlasLayout::from_grid(
          UVec2::new(config.tilesize.0.into(), config.tilesize.1.into()),
          config.size.0.into(),
          config.size.1.into(),
          None,
          None,
        );

        let atlas_layout = load_context.add_loaded_labeled_asset(
          "texure_atlas_layout",
          LoadedAsset::from(atlas_layout_asset),
        );

        Ok(CharacterBehavior {
          config,
          sprite,
          atlas_layout,
        })
      }
      #[allow(unreachable_patterns)]
      _ => Err(BehaviorLoaderError::unexpected_type("character", config.type_name())),
    }
  }
}

#[derive(Debug, Error)]
pub enum BehaviorLoaderError {
  #[error("Powermove Lib Error: {0}")]
  Lib(#[from] lib::PowermoveLibError),

  #[error("IO Error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Failed to load nested asset directly: {0}")]
  LoadDirect(#[from] bevy::asset::LoadDirectError),

  #[error("Unexpected behavior type {1}, expected {0}")]
  UnexpectedBehaviorType(String, String),
}

impl BehaviorLoaderError {
  pub fn unexpected_type(expected: &str, got: &str) -> Self {
    Self::UnexpectedBehaviorType(expected.to_string(), got.to_string())
  }
}

async fn load_behavior<'a>(reader: &'a mut Reader<'_>) -> Result<BehaviorConfig, BehaviorLoaderError> {
  let mut src = String::new();
  reader.read_to_string(&mut src).await?;
  Ok(lib::behaviors::from_str(&src)?)
}
