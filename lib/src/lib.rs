use glam::Vec3;

pub mod geometry;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn generate_terrain(verts: &Vec3) -> Result<crate::geometry::Mesh> {
  todo!("Implement terrain generation")
}
