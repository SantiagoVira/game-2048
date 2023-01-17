use rand::Rng;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
  let mut board = Board::new();

  loop {
    board.fill_square();
    board.draw();

    let dir = get_user_dir();
    board.shift(dir);

    if board.check_is_lose() { break; }
  }

  println!("Better luck next time kiddo!")
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

  // fill_square will mutate self, hence a mut ref
  pub fn fill_square(&mut self) {
    let mut rng = rand::thread_rng();

    let mut x: usize;
    let mut y: usize;

    if self.check_is_full() { return; }

    loop {
      x = rng.gen_range(0..4);
      y = rng.gen_range(0..4);
      if self.squares[y][x] == 0 { break; }
    }
    let num = rng.gen_range(1..3) * 2;
  
    self.squares[y][x] = num;
  }

  pub fn shift(&mut self, dir: &str){
    let res = match dir{
      "Up" => self.shift_bottom_to_top(),
      "Right" => self.shift_left_to_right(),
      "Down" => self.shift_top_to_bottom(),
      "Left" => self.shift_right_to_left(),
      &_ => [[0; 4]; 4]
    };

    self.squares = res;
  }

  pub fn shift_left_to_right(&self) -> [[u32; 4]; 4]{
    let mut new_squares = [[0u32; 4]; 4];
    for row_num in 0..4{
      new_squares[row_num] = shift_row(self.squares[row_num], true);
    }

    return new_squares
  }

  pub fn shift_right_to_left(&self) -> [[u32; 4]; 4]{
    let mut new_squares = [[0u32; 4]; 4];
    for row_num in 0..4{
      new_squares[row_num] = shift_row(self.squares[row_num], false);
    }

    return new_squares
  }

  pub fn shift_bottom_to_top(&self) -> [[u32; 4]; 4]{
    let mut new_squares = [[0u32; 4]; 4];
    for col_num in 0..4{
      let mut col = [0, 0 ,0 ,0];
      for j in 0..4{
        col[j] = self.squares[j][col_num];
      }
      let final_col = shift_row(col, false);
      for j in 0..4{
        new_squares[j][col_num] = final_col[j];
      }
    }

    return new_squares
  }

  pub fn shift_top_to_bottom(&self) -> [[u32; 4]; 4]{
    let mut new_squares = [[0u32; 4]; 4];
    for col_num in 0..4{
      let mut col = [0, 0 ,0 ,0];
      for j in 0..4{
        col[j] = self.squares[j][col_num];
      }
      let final_col = shift_row(col, true);
      for j in 0..4{
        new_squares[j][col_num] = final_col[j];
      }
    }

    return new_squares
  }

  pub fn check_is_full(&self) -> bool {
    for y in 0..4{
      for x in 0..4{
        if self.squares[y][x] == 0 { return false; }
      }
    }
    return true;
  }

  pub fn check_is_lose(&self) -> bool {
    for dir in ["Up", "Right", "Down", "Left"]{
      let res = match dir{
        "Up" => self.shift_bottom_to_top(),
        "Right" => self.shift_left_to_right(),
        "Down" => self.shift_top_to_bottom(),
        "Left" => self.shift_right_to_left(),
        &_ => [[0; 4]; 4]
      };

      for y in 0..4{
        for x in 0..4{
          if self.squares[y][x] != res[y][x]{ 
            // println!("({}, {}) would go from {} to {} when pushed {}", x, y, self.squares[y][x], res[y][x], dir);
            return false; 
          }
        }
      }

    }
    return true;
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
  