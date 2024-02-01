# References

## Borrowing references
- Access data without taking ownerhsip of it
- Create references using the borrow operator: `&`
```
fn main() {
  let rocket_fuel: String = String::from("RP-1");
  let length = process_fuel(&rocket_fuel);
  println!("rocket_fuel is {} and legnth is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &String) -> usize {
  println!("processing propellant {}...", propellant);
  let length = propellant.len();
  length
}
```
When working with data types that are stored on the Heap, it's more common to use a reference than passing ownership

## Mutable references
- Used when changing the value of a borrowed variable
```
fn main() {
  let mut rocket_fuel: String = String::from("RP-1");
  let length = process_fuel(&mut rocket_fuel);
  println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &mut String) -> usize {
  println!("processing propellant {}...", propellant);
  propellant.push_str(" is highly flammable");
  let length = propellant.len();
  length
}
```

### Caveats
- When using a mutable reference, you cannot create other references to it within that scope. This prevents data races.
```
// Fine
let ref1 = &mut var;

// Fine
let ref1 = &var;
let ref2 = &var;

// Bad
let ref1 = &mut var;
let ref2 = &var;
```

## Dangling references
Returning a reference can sometimes cause issues.
```
fn main() {
  let rocket_fuel = produce_fuel();
  println!("rocket_fuel is {}", rocket_fuel);
}

fn produce_fuel() -> &String {
  let new_fuel = String::from("RP-1");
  &new_fuel
}
```
The above code won't compile. This is due to the `new_fuel` variable going out of scope once the `produce_fuel()` method has run. Since the variable will no longer be in memory, it'll leave a dangling reference.

## Slices
Used when referencing a subset of a collection (e.g. an Array or String).
```
fn main() {
  let message = String::from("Greetings from Earth!");
  println!("message is {}", message);

  let last_word = &message[15..];
  println!("last_word is {}", last_word);
}
```

### String slices
- Length is in bytes
- Range indices must occur at valid UTF-8 character boundaries

#### As function parameters
```
fn main() {
  let message = String::from("Greetings from Earth!");
  let first_word = get_first_word(&message[10..]);
  println!("first_word is {}", first_word);
}

fn get_first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (index, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..index]; // found space
    }
  }

  &s // no spaces found
}
```