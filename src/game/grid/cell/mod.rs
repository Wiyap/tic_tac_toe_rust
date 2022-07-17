#[derive(Clone)]

pub struct Cell {
  pub state: char
}

impl Cell {
  pub fn init_empty() -> Cell {
    Cell {state: ' '}
  }

  pub fn is_empty(&self) -> bool {
    self.state == ' '
  }

  pub fn change_state(&mut self, new_state: char) {
    self.state = new_state;
  }
}