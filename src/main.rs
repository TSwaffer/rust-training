fn main() {
  let mut stuff: (i32, f64, char) = (10, 3.14, 'x');
  stuff.0 += 3;
  let first_item = stuff.0;

  let (a,b,c) = stuff;
  println!("{}", b);
}
