# Rust learnings
Learnings and notes gathered from https://www.linkedin.com/learning/rust-essential-training

## Primitive data types

### Guidelines
- If you need to store a fractional value, use the default f64
- For integers, consider the maximum possible value of the variable

### Integer data types
- Signed allows negative values
- Unisgned are only positive values

| Length  | Signed | Unsigned |
|-------- |--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

### Floating-point data types
- `f32` - single precision
- `f64` - double precision

```
y: 32 = 10.1234567891234567890
  -> 10.123457

y: 64 = 10.1234567891234567890
  -> 10.123456789123457
```

### Booleans
- 1 = `true`
- 0 = `false`

```
let a = true;
let b = false;

println!("a is {a} and b is {b}");
println!("NOT a is {} ", !a);
println!("a AND b is {} ", a & b);
println!("a OR b is {} ", a | b);
println!("a XOR b is {} ", a ^ b);

  a is true and b is false
  NOT a is false 
  a AND b is false 
  a OR b is true 
  a XOR b is true 
```

#### Short-circuiting logical operations
More efficient 

- false && [not evaluated] = false
- true || [not evaluated] = true

### Casting
Converting one data type to another:

```
let a: u8 = 10;
let b: f32 = 3.0;
let c: f32 = a as f32 / b;
```

Consider data loss when casting:
```
3 as f64 -> 3.0 // Okay
3.9 as i32 -> 3 // Bad. Casting truncates, not round
300 as u8 -> 44 // Bad. Max value of u8 is 255 so extra is rolled over
-300 as u32 -> 4294966996 // Bad
```

## Formatting print statements
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

## Bitwise operations
Commonly used for low level programming when manipulating bits on an individual basis.

```
let value = 0b1111_0101u8;

println!("value is {value}");
println!("value is {:08b}", value);
  -> value is 245
  -> value is 11110101

// Bitwise NOT
value = !value;
-> value is 00001010

// Bitwise AND
value = value & 0b1111_0111;
-> value is 00000010

// Bitwise OR
value = value | 0b0100_0000;
 -> value is 01000010

 // Bitwise XOR (0 if both values are same, 1 if not)
 value = value ^ 0b0101_0101;
-> value is 00010111

// Bitwise shift
value = value << 4; // left
-> value is 01110000

value = value >> 2; // right
-> value is 00011100
```

## Compound data types

### One dimensional arrays
- Fixed length (cannot be resized)
- All elements must have same data type
- Stored in order

```
// Declaring thw whole thing
let letters: [char; 3] = ['a', 'b', 'c']; 

// Declaring an empty array that stores i32 integers with a length of 5
let numbers: [i32; 5];

// sets the variable to be populated with 5 0s
numbers = [0; 5];

// Getting a length of an array
let length: usize = numbers.len();
  => 5
```

### Multidimensional arrays
- All elements must have same type
- Sub arrays must be same size

#### Two dimensional arrays
```
let numbers: [[i32; 3]; 2] = [[1,2,3], [4,5,6]];
let number: i32 = numbers[0][1];
println!("{}", number);
  => 2
```

#### Three dimensional arrays
```
// An array of 100 by 20 by 5 elements, all equalling 0
let numbers: [[[i32; 100]; 20]; 5] = [[[0; 100]; 20]; 5];
```

### Tuple
- Can hold a mix of data types

```
// Declaring a tuple
let mut stuff: (i32, f64, char) = (10, 3.14, 'x');
// Changing a value
stuff.0 += 3;
// Accessing a value
let first_item = stuff.0;

// Destructuring
let (a,b,c) = stuff;
println!("{}", b);
  => 3.14
```

## Functions
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

## Program flow control

### Conditional execution
```
let x = 3;
let y = 5;

if x > y {
  print!("x is greater than y");
} else if x < y{
  print!("x is less than y");
} else {
  print!("x is equal to y");
}
```

### Conditional assignment
```
let make_x_odd = true;
let x = if make_x_odd { 1 } else { 2 };
```

### Loops
```
let mut x = 0;

// Will run indefinitely until break
let result = loop {
  if x == 10 {
    // returns x multiplied by 10
    break x * 10;
  }

  x += 1;
  println!("count is {}", x);
};

println!("After loop");
println!("result is {}", result);
  => "result is 100"

// while loop - cannot return value
let mut x = 0;

while x < 10 {
  x += 1;
  println!("{}", x);
}

// for loop
let message = ['h', 'e', 'l', 'l', 'o'];

// .iter().enumerate() allows you to get the index
for (index, item) in message.iter().enumerate() {
  println!("item {} is {}", index, item);
}
=> item 0 is h
   item 1 is e
   item 2 is l
   item 3 is l
   item 4 is o

// Using a range
for number in 0..5 {
  println!("number is {}", number);
}
=> number is 0
   number is 1
   number is 2
   number is 3
   number is 4
```

#### Cases for using each type of loop
- `loop`
  - Repeat a block of code forever
  - Need to return a value
- `while`
  - Continue repeating block of code as long as a confition is true
- `for`
  - Iterate over each item in a collection
  - Repeat a block of code N times -> iterate over range 0..N

#### Nested loops example

```
let matrix: [[i32; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];

for row in matrix {
  for num in row {
    print!("{}\t", num);
  }
  println!();
}

=>
  1       2       3
  4       5       6
  7       8       9

// Mutatable version
let mut matrix: [[i32; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];

for row in matrix.iter_mut() {
  for num in row.iter_mut() {
    *num += 10;
    print!("{}\t", num);
  }
  println!();
}
```
