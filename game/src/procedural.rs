use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Vertex {
  pub pos: Vec3,
  pub normal: Vec3,
  pub uv: Vec2,
}
