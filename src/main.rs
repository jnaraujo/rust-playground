use std::io;

fn main() {
  println!("Choose a letter:");
  let mut letter = String::new();

  io::stdin()
    .read_line(&mut letter)
    .expect("Failed to read letter");

  println!("You choosed ther letter: {letter}");

  println!("Address of argument is {:p}", &letter);

  memo(&mut letter);

  println!("Value: {letter}");
  println!("Address of argument is {:p}", &letter);
}

fn memo(s: &mut String){
  println!("Address of argument is {:p}", s);
  *s = String::from("oi");

}