# Input and Output

## Command-line arguments
Use `std::env::args`.

Common uses:
- File paths
- Configuration settings

```
use std::env;

fn main() {
  if env::args().len() <= 2 {
    println!("Progrma requires at least 2 arguments.");
    return;
  }

  for(index, argument) in env::args().enumerate() {
    println!("argument {} is {}", index, argument);
  }

  let arg2 = env::args().nth(2).unwrap();
  println!("arg2 is {}", arg2);
}
```

## Reading from files
Use `std:fs`.

```
use std::fs;

fn main() {
  // Read to a string
  let contents = fs::read_to_string("planets.txt").unwrap();
  println!("contents is {}", contents);

  for line in contents.lines() {
      println!("line is {}", line);
  }

  // Read as bytes
  let contents = fs::read("planets.txt").unwrap();
  println!("contents is {:?}", contents);
}
```

## Writing to files
Simplest way is to use `std::fs::write`. However, there are some things to keep in mind:
- It will replace contents of existing files
- Writes entire contents of the file

```
use std::fs;

fn main() {
  let mut speech = String::new();
  speech.push_str("We choose to go to the Moon in this decade\n");
  speech.push_str("and do other things,\n");
  speech.push_str("not because they are easy,\n");
  speech.push_str("but beacuse they are hard.");

  fs::write("speech.txt", speech);
}
```

### Appending
You'll need to use `std::io::Write` and create a file variable with the options to add to it.
```
let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
file.write(b"\nPluto");
```

