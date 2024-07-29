use glam::Vec3;
use thiserror::Error;

pub mod behaviors;
pub mod directions;
pub mod geometry;

pub type Result<T> = std::result::Result<T, PowermoveLibError>;

#[derive(Debug, Error)]
pub enum PowermoveLibError {
  #[error("PowermoveLib Parse Error: {0}")]
  ParseError(String),

  #[error("PowermoveLib Generic Error: {0}")]
  GenericError(String),
}

impl PowermoveLibError {
  pub fn parse_error(err: impl std::error::Error) -> Self {
    Self::ParseError(err.to_string())
  }

  pub fn generic(err: impl Into<String>) -> Self {
    Self::GenericError(err.into())
  }
}

pub fn generate_terrain(verts: &Vec3) -> Result<crate::geometry::Mesh> {
  todo!("Implement terrain generation")
}
