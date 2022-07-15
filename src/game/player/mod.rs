#[derive(Copy, Clone, PartialEq)]

pub struct Player {
  pub sign: char,
}

impl Player {
  pub fn new(sign: char) -> Player {
    Player {sign: sign}
  }
}