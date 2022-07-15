mod grid;

use grid::Grid;

fn main() {
  let mut grid = Grid::generate();

  grid.display()
  // println!("{}", grid.cells[0].state);
}
