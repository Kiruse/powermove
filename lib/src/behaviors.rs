use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::PowermoveLibError;
use crate::Result;

/// This type is primarily used for serde/yml de/serialization. The actual Asset as used in the game
/// is in the other project to minimize dependencies of this library.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Behavior {
  #[serde(rename = "character")]
  Character(CharacterBehavior),
}

impl Behavior {
  pub fn type_name(&self) -> &str {
    match self {
      Behavior::Character(_) => "character",
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CharacterBehavior {
  /// CharacterBehavior version
  pub version: u8,
  /// Iteration of this behavior
  pub iteration: Option<u8>,
  /// Relative path in `assets/sprites`
  pub sprite: String,
  /// width x height of each tile in pixels
  pub tilesize: (u16, u16),
  /// Number of tiles in the sprite sheet, by columns x rows
  pub size: (u16, u16),
  /// Animation configuration
  pub anims: CharacterBehaviorAnims,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CharacterBehaviorAnims {
  /// Idle animation configuration.
  pub idle: CharacterBehaviorAnim,
  /// Optional walk animation. Expects 8 series of frames with this configuration corresponding to the 8 directions, starting with north & going clockwise.
  pub walk: Option<CharacterBehaviorAnim>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct CharacterBehaviorAnim {
  /// Index of the first frame in the animation
  pub offset: u16,
  /// Number of frames in this animation
  pub stride: u16,
  /// Duration of each frame in milliseconds
  pub dt: u16,
}

pub fn load_behavior(path: &impl AsRef<Path>) -> Result<Behavior> {
  from_str(&std::fs::read_to_string(path.as_ref()).map_err(PowermoveLibError::parse_error)?)
}

pub fn from_str(source: &impl AsRef<str>) -> Result<Behavior> {
  Ok(serde_yml::from_str(&source.as_ref()).map_err(PowermoveLibError::parse_error)?)
}

//#region Assert Send/Sync
struct AssertSendSync<T: Send + Sync> {
  _marker: std::marker::PhantomData<T>,
}

impl<T: Send + Sync> AssertSendSync<T> {
  pub fn new() -> Self {
    Self {
      _marker: std::marker::PhantomData,
    }
  }
}

fn _assert_send_sync() {
  AssertSendSync::<Behavior>::new();
}
//#endregion Assert Send/Sync
