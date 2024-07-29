
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Direction {
  #[default]
  None,
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}

impl Direction {
  pub fn index(&self) -> u8 {
    match self {
      Direction::None => u8::MAX,
      Direction::North => 0,
      Direction::NorthEast => 1,
      Direction::East => 2,
      Direction::SouthEast => 3,
      Direction::South => 4,
      Direction::SouthWest => 5,
      Direction::West => 6,
      Direction::NorthWest => 7,
    }
  }
}
