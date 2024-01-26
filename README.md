# Rust learnings
Learnings and notes gathered from https://www.linkedin.com/learning/rust-essential-training

- [Primitive data types](./docs/primitive_data_types.md)

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

## Ownership

### Shadowing
Declaring a new variable with the same name as an existing variable. The new variable "shadows" the previous variable.
- Can change data type
- Can change whether it's mutable
```
fn main() {
  let planet = "Earth";
  println!("planet is {}", planet);
  let mut planet = 4;
  println!("planet is {}", planet);
}
=>
  planet is Earth
  planet is 4
```

### Program memory
Stored in two sections:
- Stack
- Heap

#### Stack
- Values stored in sequential order
- LIFO
- Quick to push and pop data
- Access data quickly
- Small size
- All data must have a known, fixed size

#### Heap
- Slower than the stack when adding and accessing
- Dynamically add and remove data
- Can store large data structures

### String data type
There are two types.

#### String literal
- Hard-coded into the executable
- Immutable
- Must be known before compilation

#### String type
- Allocated on the heap
- Mutable
- Dynamically generated at runtime

Declaring a String type looks like this:
`let message: String = String::from("Earth");`

In terms of memory, it looks like this:
```
HEAP                      STACK
| index | value |       | field | value                                |
|-------|-------|       |-------|--------------------------------------|
| 0     | E     |       | ptr   | (points to the location in the heap) |
| 1     | a     |       | len   | 5                                    |
| 2     | r     |       | cap   | 8                                    |
| 3     | t     |
| 4     | h     |
| 5     |       |
| 6     |       |
| 7     |       |
```

You can change the variable by marking it as `mut` and using `.push_str("text");`
```
let mut message: String = String::from("Earth");
message.push_str(" is home");
```

### Ownership
Rust's handling of memory management. Variables are responsible for freeing their own resources, based on the following rules:
1. Every value is "owned" by one, and only one, variable at a time
2. When the owning variables goes out of scope, the value is dropped

This makes the code safe and efficient. However, it requires understanding of ownership.

#### Moving data
This is the process of moving ownership of a variable. The below code is valid.
```
let outer_planet: String;
  {
    let mut inner_planet = String::from("Mecury");
    println!("inner_planet is {}", inner_planet);
    outer_planet = inner_planet;
  }
  println!("outer_planet is {}", outer_planet);
```
However, the code below is not valid.
```
let outer_planet: String;
  {
    let mut inner_planet = String::from("Mecury");
    outer_planet = inner_planet;
    println!("inner_planet is {}", inner_planet);
  }
  println!("outer_planet is {}", outer_planet);
```
The reason being the first rule of ownership (only one owner). So when you call `outer_planet = inner_planet`, Rust invalidates `inner_planet` and "moves" the value stored in the heap over to `outer_planet`.

#### Cloning data
The above example can be "fixed" by cloning the `inner_planet` variable, by using `.clone()`. This results in two seperate pieces of data stored in the Heap, with each variable pointing to a single one.
```
let outer_planet: String;
  {
    let mut inner_planet = String::from("Mecury");
    outer_planet = inner_planet.clone();
    println!("inner_planet is {}", inner_planet);
  }
  println!("outer_planet is {}", outer_planet);
```
This means that affecting one variable doesn't change the other.

#### Copying data
The above examples only are needed for varaibles that have data stored in the Heap. For stack-only data types, like integer and floating point, you can just assign. Copying occurs implicitly, whereas cloning happens explicitly.
```
let outer_planet: i32;
  {
    let mut inner_planet = 1;
    outer_planet = inner_planet;
    inner_planet += 1;
    println!("inner_planet is {}", inner_planet);
  }
  println!("outer_planet is {}", outer_planet);
```

#### Transferring ownership
Another gotcha is when using functions and Heap data type variables. The code below throws an error.
```
fn main() {
  let rocket_fuel = String::from("RP-1");
  process_fuel(rocket_fuel);
  println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) {
  println!("processing propellant {}...", propellant);
}
```
This happens because the `rocket_fuel` variable doesn't implicitly make a copy of the data. So it's transferring ownership of the `rocket_fuel` variable to the `propellant` variable.

One solution would be to use the `clone()` method. This would keep `rocket_fuel` as the owner of the original data in the Heap, and create a new set of data in the Heap that `propellant` would be the owner of. However, this may not always be want you want.

You can make the function return a value to pass ownership back to the original variable.
```
fn main() {
  let rocket_fuel = String::from("RP-1");
  let rocket_fuel = process_fuel(rocket_fuel);
  println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
  println!("processing propellant {}...", propellant);
  propellant
}
```
