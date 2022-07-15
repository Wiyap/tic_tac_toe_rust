mod player;
mod grid;

use player::Player;
use grid::Grid;

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
    self.grid.cells[0].change_state(self.current_player.sign);
    if self.current_player == self.player_1 {
      self.current_player = self.player_2
    } else {
      self.current_player = self.player_1
    };

    self.grid.display()
  }
}