# Formatting print statements
https://doc.rust-lang.org/std/fmt/index.html

```
// Simple example
println!("c is {}", c);
  -> c is 3.3333333

// Floating-point formatting
println!("c is {:.3}", c);
  -> c is 3.333

// Ensuring the message is exactly 8 chars
println!("c is {:8.3}", c);
  -> c is    3.333

// Ensuring the message is exactly 8 chars with 0s added in the leading space
println!("c is {:08.3}", c);
  -> c is 0003.333

// Printing multiple variables
println!("c is {:08.3}\na is {}", c, a);
  -> c is 0003.333
  -> a is 10

// Positional parameters
println!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);
  -> c is 0003.333
  -> a is 10
  -> once again, c is 3.3333333
```