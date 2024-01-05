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

// Bitewise OR
value = value | 0b0100_0000;
 -> value is 01000010

 // Bitewise XOR (0 if both values are same, 1 if not)
 value = value ^ 0b0101_0101;
-> value is 00010111

// Bitewise shift
value = value << 4; // left
-> value is 01110000

value = value >> 2; // right
-> value is 00011100
```