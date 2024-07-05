use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::procedural::Vertex;

#[derive(Component, Serialize, Deserialize, Debug, PartialEq)]
pub struct Island {
  pub name: String,
  pub tiles: SparseCubeMap,
}
