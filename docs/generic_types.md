# Generic types

- Abstract stand-ins for concrete data types or other properties
- Can be used with structs, functions, methods, and more
- Defined with `<T>`

```
struct Rectangle<T, U> {
  width: T,
  height: U
}

// Methods for generic types
impl <T,U> Rectangle<T,U> {
  fn get_width(&self) -> &T {
    &self.width
  }
}

// Methods for a specific type
impl Rectangle<u8,u8> {
  fn get_perimeter(&self) -> u8 {
    (2 * self.width) + (2*self.height)
  }
}

fn main() {
  let rect = Rectangle {
    width: 1u8,
    height: 3u8
  };

  println!("width is {}", rect.get_width());
  println!("perimiter is {}", rect.get_perimeter());
}
```

## Box<T> Data Type
- Store data on the heap instead of the stack
- Provide additional functionality beyond references
- Box<T> has ownership of the data it points to

### Use cases
- Store a type whose size cannot be known at compile time (e.g. Recursive types).
- Tranfer ownership of data rather than copt it on the stack. Avoid copying large amounts of data.
