use rand::Rng;

fn main() {
  let mut board = Board::new();

  board.fill_square();
  board.draw();
  
}

struct Board {
  squares: [[u32; 4]; 4],
}

impl Board {
  pub fn new() -> Self {
    // create the board
    return Board{squares: [[0u32; 4]; 4]};
  }  

  // draw shouldnt need to mutate the board, hence an immut. ref to self
  pub fn draw(&self) {
    for y in 0..4{
      println!("{}", "-".repeat(7 * 4 + 1));
      for x in 0..4{
        if self.squares[y][x] != 0 {
          print!("| {0: <5}", self.squares[y][x]);
        } else {
          print!("|      ");
        }
      }
      println!("|");
    }
    
    println!("{}", "-".repeat(7 * 4 + 1));
  }

  // fill_squares will mutate self, hence a mut ref
  pub fn fill_square(&mut self) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0..4);
    let y = rng.gen_range(0..4);
    let num = rng.gen_range(1..3) * 2;
  
    self.squares[y][x] = num;
  }
}
