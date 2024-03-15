# Error handling

Rust has several features to handle runtime errors. These errors are grouped into two categories - recoverable and unrecoverable. Recoverable errors are something that your program can reasonably do something to resolves (e.g. file not found), whereas unrecoverable is something more like index beyond array bounds.

Rust does not have exceptions in the traditional programming sense, instead it has:
- `Result<T,E>` for recoverable errors
- `panic!` for unrecoverable errors

The `panic!` macro immediately terminates the program and provides feedback about what went wrong.

## Recoverable errors
As mentioned above, these are errors that do not cause the program to fail and can be corrected. To faciitate handling those types of recoverable errors, Rust defines the `Result` enum with two variants, `Ok` and `Err`, representing the result of an operation that can either succeed or fail.

```
enum Result<T,E> {
  Ok(T),
  Err(E)
}
```

### Using `match`
By using `match` with recoverable errors, you can write more resilient programs:
```
use std::fs;
use std::io;

fn main() {
  let result: Result<String, io::Error> = fs::read_to_string("non_existant_file.txt");

  let contents: String = match result {
    Ok(message) => message,
    Err(error) => match error.kind() {
      io::ErrorKind::NotFound => String::from("File not found"),
      io::ErrorKind::PermissionDenied => String::from("Permission denied"),
      _ => panic!("Another type of error: {:?}", error)
    }
  };
}
```