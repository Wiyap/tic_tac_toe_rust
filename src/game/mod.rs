mod player;
mod grid;

use player::Player;
use grid::Grid;
use std::io::{stdin,stdout,Write};

pub struct Game {
  pub player_1: Player,
  pub player_2: Player,
  current_player: Player,
  grid: Grid,
}

impl Game {
  pub fn new() -> Game {
    let player_1 = Player::new('O');
    let player_2 = Player::new('X');

    Game {
      player_1: player_1,
      player_2: player_2,
      current_player: player_1,
      grid: Grid::generate(),
    }
  }

  pub fn turn(&mut self) {
    self.grid.display();
    let mut user_input = String::new();
    stdin().read_line(&mut user_input);

    let mut index = Self::user_input_to_index(&user_input);
    let mut cell = &mut self.grid.cells[index];

    while !cell.is_empty() {
      stdin().read_line(&mut user_input);
      index = Self::user_input_to_index(&user_input);
      cell = &mut self.grid.cells[index]
    };

    cell.change_state(self.current_player.sign);

    if self.current_player == self.player_1 {
      self.current_player = self.player_2
    } else {
      self.current_player = self.player_1
    };

    self.grid.display()           
  }

  fn format_user_input(mut user_input: String) -> String {
    if let Some('\n') = user_input.chars().next_back() {
      user_input.pop();
    }
    if let Some('\r') = user_input.chars().next_back() {
      user_input.pop();
    }
    user_input
  }

  fn user_input_to_index(mut user_input: &String) -> usize {
    let formated_input: &str = &*Self::format_user_input(user_input.to_string());
    
    match formated_input {
      "A1" => return 0,
      "A2" => return 1,
      "A3" => return 2,
      "B1" => return 3,
      "B2" => return 4,
      "B3" => return 5,
      "C1" => return 6,
      "C2" => return 7,
      "C3" => return 8,
      _ => return 0
    }
  }
}