# Enums
```
#[derive(Debug)]
enum Shape {
  Circle(f64),
  Rectangle(f64, f64),
  Triangle(f64, f64, f64)
}

fn main() {
  let my_shape = Shape::Rectangle(1.2, 3.4);
  println!("my shape is {:?}", my_shape);
}
```

## Match operator
Compares a value to a series of patterns to determine which code to execute. Like a `switch` statement.

```
match my_shape {
  Shape::Circle(r) => println!("Circle iwith radious {}", r),
  Shape::Rectangle(w, h) => print!("{} x {} rectangle", w, h),
  Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c)
}
```

### Default placeholder
```
// within match
_ => {
  println!("{} did not match", my_shape);
  "something else"
}
```

## Enum methods
```
impl Shape {
  fn get_perimeter(&self) -> f64 {
    match self {
      Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
      Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
      Shape::Triangle(a, b, c) => a +  b+ c
    }
  }
}

my_shape.get_perimeter();
```

