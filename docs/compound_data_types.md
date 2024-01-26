# Compound data types

## One dimensional arrays
- Fixed length (cannot be resized)
- All elements must have same data type
- Stored in order

```
// Declaring thw whole thing
let letters: [char; 3] = ['a', 'b', 'c']; 

// Declaring an empty array that stores i32 integers with a length of 5
let numbers: [i32; 5];

// sets the variable to be populated with 5 0s
numbers = [0; 5];

// Getting a length of an array
let length: usize = numbers.len();
  => 5
```

## Multidimensional arrays
- All elements must have same type
- Sub arrays must be same size

### Two dimensional arrays
```
let numbers: [[i32; 3]; 2] = [[1,2,3], [4,5,6]];
let number: i32 = numbers[0][1];
println!("{}", number);
  => 2
```

### Three dimensional arrays
```
// An array of 100 by 20 by 5 elements, all equalling 0
let numbers: [[[i32; 100]; 20]; 5] = [[[0; 100]; 20]; 5];
```

## Tuple
- Can hold a mix of data types

```
// Declaring a tuple
let mut stuff: (i32, f64, char) = (10, 3.14, 'x');
// Changing a value
stuff.0 += 3;
// Accessing a value
let first_item = stuff.0;

// Destructuring
let (a,b,c) = stuff;
println!("{}", b);
  => 3.14
```