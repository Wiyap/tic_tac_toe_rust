mod cell;

use cell::Cell;

pub struct Grid {
  pub cells: Vec<Cell>
}

impl Grid {
  pub fn generate() -> Grid {
    Grid {cells: [
        Cell::init_empty(),
        Cell::init_empty(),
        Cell::init_empty(),
        Cell::init_empty(),
        Cell::init_empty(),
        Cell::init_empty(),
        Cell::init_empty(),
        Cell::init_empty(),
        Cell::init_empty(),
      ].to_vec()
    }
  }

  pub fn display(&self) {
    println!("|{}|{}|{}|", self.cells[0].state, self.cells[1].state, self.cells[2].state);
    println!("|{}|{}|{}|", self.cells[3].state, self.cells[4].state, self.cells[5].state);
    println!("|{}|{}|{}|", self.cells[6].state, self.cells[7].state, self.cells[8].state);
  }
}