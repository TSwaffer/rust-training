# Functions
```
fn main() {
  say_hello();
}

fn say_hello() {
  println!("hello");
}

// With a parameter
fn say_number(number: i32) {
  println!("{}", number);
}

// A function that returns a value
fn main() {
  let result = square(13);
  println!("{}", result);
}

// -> determines the output value's type
fn square(x: i32) -> i32 {
  x * x // notice the lack of semi-colon
}
```