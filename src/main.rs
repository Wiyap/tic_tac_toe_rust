mod game;

use game::Game;

fn main() {
  let mut game = Game::new();
  while true {
    game.turn();
  }
  // println!("{}", game.current_player.sign);
}
