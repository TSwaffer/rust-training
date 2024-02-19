use std::env::args;
use std::fs;

fn main() {
  if args().len() <= 2 {
    println!("Program requires at least 2 arguments - file path and name");
    return;
  }

  let file_path: String = args().nth(1).unwrap();
  let name: String =  args().nth(2).unwrap();

  let contents = fs::read_to_string(&file_path).unwrap();
  let is_in_contents = contents.contains(&name);

  if is_in_contents {
    println!("Found {} in {}", name, file_path);
  } else {
    println!("This name doesn't exist in {}", file_path);
  }
}