use rand::Rng;

fn main() {
  let mut state = [[0u8; 4]; 4];

  fill_square(&mut state);

  println!("{:#?}", state);


  draw()
  
}

fn fill_square(state: &mut [[u8; 4]; 4]){
  let mut rng = rand::thread_rng();

  let x = rng.gen_range(0..4);
  let y = rng.gen_range(0..4);
  let num = rng.gen_range(1..3) * 2;

  state[y][x] = num;
}


fn draw() {
  for y in 0..4{
    println!("{}", "-".repeat(7 * 4 + 1));
    for i in 0..4{
      print!("| {0: <5}", i + y * 4);
    }
    println!("|");
  }
  
  println!("{}", "-".repeat(7 * 4 + 1));
}