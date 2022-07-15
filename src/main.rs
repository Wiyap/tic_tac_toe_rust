mod game;

use game::Game;

fn main() {
  let mut game = Game::new();
  game.turn();
  // println!("{}", game.current_player.sign);
}
