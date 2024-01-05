# Rust learnings

## Primitive data types

### Guidelines
- If you need to store a fractional value, use the default f64
- For integers, consider the maximum possible value of the variable

### Integer data types
Signed allows negative values
Unisgned are only positive values

| Length  | Signed | Unsigned |
|-------- |--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

### Floating-point data types
f32 - single precision
f64 - double precision

```
y: 32 = 10.1234567891234567890
  -> 10.123457

y: 64 = 10.1234567891234567890
  -> 10.123456789123457
```

