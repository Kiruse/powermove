use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::{PowermoveLibError, Result};

/** A Geometry is an entity that contains vertices. Every set of 3 vertices forms a face. */
pub trait Geometry {
  fn vertices(&self) -> Vec<Vertex>;
}

pub trait GeometryMethods {
  fn build(&self) -> Result<GeometryCache>;
}

impl<T: Geometry> GeometryMethods for T {
  fn build(&self) -> Result<GeometryCache> {
    let verts = self.vertices();
    if verts.len() % 3 != 0 {
      return Err(PowermoveLibError::generic("Invalid number of vertices"));
    }

    let mut pos: Vec<Vec3> = vec![];
    let mut normal: Vec<Vec3> = vec![];
    let mut uv: Vec<Vec2> = vec![];

    for vertex in self.vertices() {
      pos.push(vertex.pos);
      normal.push(vertex.normal);
      uv.push(vertex.uv);
    }

    Ok(GeometryCache { pos, normal, uv })
  }
}

pub struct GeometryCache {
  pub pos: Vec<Vec3>,
  pub normal: Vec<Vec3>,
  pub uv: Vec<Vec2>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vertex {
  pub pos: Vec3,
  pub normal: Vec3,
  pub uv: Vec2,
}

/** The most fundamental type of mesh. */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Mesh {
  pub vertices: Vec<Vertex>,
}

impl Geometry for Mesh {
  fn vertices(&self) -> Vec<Vertex> {
    self.vertices.clone()
  }
}
