# Bitwise operations
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

// Bitwise OR
value = value | 0b0100_0000;
 -> value is 01000010

 // Bitwise XOR (0 if both values are same, 1 if not)
 value = value ^ 0b0101_0101;
-> value is 00010111

// Bitwise shift
value = value << 4; // left
-> value is 01110000

value = value >> 2; // right
-> value is 00011100
```