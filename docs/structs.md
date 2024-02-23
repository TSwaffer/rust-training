# Structs

- Group multiple items of mixed data types
- Elements are named

```
struct Shuttle {
  name: String,
  crew_size: u8,
  propellant: f64
}

fn main() {
  let vehicle = Shuttle {
    name: String::from("Endeavour"),
    crew_size: 7,
    propellant: 83598.5
  };

  println!("name is {}", vehicle.name);
}
```

Struct data is stored on the stack, unless said otherwise.

## Struct methods

- Subroutine associated with a struct
- Can have input parameters and a return value
- Declared using the `fn` keyword
- First parameter is a reference to the struct instance

```
struct Shuttle {
  name: String,
  crew_size: u8,
  propellant: f64
}

impl Shuttle {
  fn get_name(&self) -> &str {
    &self.name
  }

  fn add_fuel(&mut self, gallons: f64) {
    self.propellant += gallons
  }
}
```

## Assoicated functions
- Functions assocaiated with a struct data type
- Does not have a `&self` parameter

This makes them useful for creating constructor methods

```
impl Shuttle {
  fn new(name: &str) -> Shuttle {
    Shuttle {
      name: String::from(name), 
      crew_size: 7,
      propellant: 0.0
    }
  }
}

Shuttle::new("Tom's speedy shuttle");
```
## Tuple structs
- Store a collection of mixed data without named fields
- Distinguishable as a unique data type

```
struct Colour (u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
  p.1
}

fn main() {
  let red = Colour(255, 0, 0);
  println!("First value is {}", red.0);

  let coord = Point(4,5,6);
  let y = get_y(coord);
  println!("y is {}", y);
}
```