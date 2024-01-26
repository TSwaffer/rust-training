# Ownership

## Shadowing
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

## Program memory
Stored in two sections:
- Stack
- Heap

### Stack
- Values stored in sequential order
- LIFO
- Quick to push and pop data
- Access data quickly
- Small size
- All data must have a known, fixed size

### Heap
- Slower than the stack when adding and accessing
- Dynamically add and remove data
- Can store large data structures

## String data type
There are two types.

### String literal
- Hard-coded into the executable
- Immutable
- Must be known before compilation

### String type
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

## Ownership
Rust's handling of memory management. Variables are responsible for freeing their own resources, based on the following rules:
1. Every value is "owned" by one, and only one, variable at a time
2. When the owning variables goes out of scope, the value is dropped

This makes the code safe and efficient. However, it requires understanding of ownership.

### Moving data
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

### Cloning data
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

### Copying data
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

### Transferring ownership
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