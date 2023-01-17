use std::io::Error;

use rand::Rng;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};

fn main() {
  let mut board = Board::new();

  board.fill_square();
  board.fill_square();
  board.fill_square();
  board.fill_square();
  board.fill_square();
  board.fill_square();
  board.fill_square();
  board.fill_square();
  board.fill_square();

  board.draw();
  board.shift_left_to_right();

  board.draw();

  let dir = get_user_dir();
  println!("{}", dir);
}

fn get_user_dir() -> &'static str {

  let items = vec!["Up", "Right", "Down", "Left"];
  let selection = Select::with_theme(&ColorfulTheme::default())
      .items(&items)
      .with_prompt("Choose your direction")
      .default(0)
      .interact()
      .unwrap();

  return items[selection];
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

    let mut x: usize;
    let mut y: usize;

    loop {
      x = rng.gen_range(0..4);
      y = rng.gen_range(0..4);
      if self.squares[y][x] == 0 { break; }
    }
    let num = rng.gen_range(1..3) * 2;
  
    self.squares[y][x] = num;
  }

  pub fn shift_left_to_right(&mut self){
    // Shift right
    for row_num in 0..4{
      self.squares[row_num] = shift_row(self.squares[row_num], true);
    }
  }

  pub fn shift_right_to_left(&mut self){
      // Shift left
      for row_num in 0..4{
        self.squares[row_num] = shift_row(self.squares[row_num], false);
      }
    }
    
  }
  
fn shift_row(mut start_row: [u32; 4], rev: bool) -> [u32; 4]{
    // Shift left
      let mut new_row = [0,0,0,0];
      let mut final_row = [0, 0, 0, 0];
      let mut idx = 0;

      if rev {start_row.reverse();}
      
      for num in start_row{
        if num != 0{
          new_row[idx] = num;
          idx = (idx + 5) % 4;
        }
      }

      for i in (0..4).rev(){
        if i > 0 && new_row[i] == new_row[i-1]{
          new_row[i] *= 2;
          new_row[i-1] = 0;
        }
      }

      idx = 0;
      for num in new_row{
        if num != 0{
          final_row[idx] = num;
          idx = (idx + 5) % 4;
        }
      }
      
      if rev {final_row.reverse();}
      
      return final_row;
    }
  