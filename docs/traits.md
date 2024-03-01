# Traits

- A collection of methods representing a set of behaviours necessary to accomplish some task
- Data types can implement a trait
- Generics use traits to specify the capabilities of unknown data types
- Similar to interfaces in other programming languages

```
struct Satellite {
  name: String,
  velocity: f64 // miles per second
}

struct SpaceStation {
  name: String,
  crew_size: u8,
  altitude: u32 // miles
}

trait Description {
  fn describe(&self) -> String;
}

impl Description for Satellite {
  fn describe(&self) -> String {
      format!("the {} flying at {} mile per second", self.name, self.velocity)
  }
}

impl Description for SpaceStation {
  fn describe(&self) -> String {
      format!("the {} flying at {} mile high with {} crew memebers", self.name, self.altitude, self.crew_size)
  }
}
```

## Default implementation
Is useful when you have a trait that is implemented in many places.

```
trait Description {
  fn describe(&self) -> String {
    String::from("an object flying through space")
  }
}
```

## Derive traits
Provides default implementations for several common traits.

```
#[derive(PartialEq, PartialOrd)]
struct Satellite {
  name: String,
  velocity: f32// miles per second
}

fn main() {
  let hubble = Satellite {
    name: String::from("Hubble telescope"),
    velocity: 4.72
  };
  let gps = Satellite {
    name: String::from("GPS"),
    velocity: 2.42
  };
  println!("hubble == gps is {}", hubble == gps);
  println!("hubble > gps is {}", hubble > gps);
}
```

## Trait bounds
- Used when you require a generic type to implement specific traits
- It guarantees the generic type will have necessary behaviours

```
use std::any;
use std::fmt;

fn print_type<T: fmt::Debug>(item: T) {
  println!("{:?} is {}", item, any::type_name::<T>());
}

fn main() {
  print_type(13);
  print_type(13.0);
  print_type("thirteen");
  print_type([13]);
}
```

## Multiple trait bounds
Adding multiple trait bounds can become messy, especially when there are multiple inputs too. So Rust makes it cleaner by using `where`.

```
use std::fmt;

// fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U) 
  where T: fmt::Display + PartialEq + From<U>, 
        U: fmt::Display + PartialEq + Copy
{
  if a == T::from(b) {
    println!("{} is equal to {}", a, b);
  } else {
    println!("{} is NOT equal to {}", a, b);
  }
}

fn main() {
  compare_and_print(1.0, 1);
  compare_and_print(1.1, 1);
}
```

## Return types with implemented traits

```
use std::fmt;

// Despite the function returning any type of value that implements `fmt::Display`,
// if this function where to be altered so it returns two different data types then
// it would not compile. 
fn get_disaplyable() -> impl fmt::Display {
  13
}

fn main() {
  println!("output is {}", get_disaplyable());
}
```