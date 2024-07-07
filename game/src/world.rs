use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct World {
  pub islands: Vec<Island>,
}

#[derive(Default, Debug)]
pub struct Island {
  pub mesh: Handle<Mesh>,
}
