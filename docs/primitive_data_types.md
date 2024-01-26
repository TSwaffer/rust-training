# Primitive data types

## Guidelines
- If you need to store a fractional value, use the default f64
- For integers, consider the maximum possible value of the variable

## Integer data types
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

## Floating-point data types
- `f32` - single precision
- `f64` - double precision

```
y: 32 = 10.1234567891234567890
  -> 10.123457

y: 64 = 10.1234567891234567890
  -> 10.123456789123457
```

## Booleans
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

### Short-circuiting logical operations
More efficient 

- false && [not evaluated] = false
- true || [not evaluated] = true

## Casting
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