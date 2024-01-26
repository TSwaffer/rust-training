# Program flow control

## Conditional execution
```
let x = 3;
let y = 5;

if x > y {
  print!("x is greater than y");
} else if x < y{
  print!("x is less than y");
} else {
  print!("x is equal to y");
}
```

## Conditional assignment
```
let make_x_odd = true;
let x = if make_x_odd { 1 } else { 2 };
```

## Loops
```
let mut x = 0;

// Will run indefinitely until break
let result = loop {
  if x == 10 {
    // returns x multiplied by 10
    break x * 10;
  }

  x += 1;
  println!("count is {}", x);
};

println!("After loop");
println!("result is {}", result);
  => "result is 100"

// while loop - cannot return value
let mut x = 0;

while x < 10 {
  x += 1;
  println!("{}", x);
}

// for loop
let message = ['h', 'e', 'l', 'l', 'o'];

// .iter().enumerate() allows you to get the index
for (index, item) in message.iter().enumerate() {
  println!("item {} is {}", index, item);
}
=> item 0 is h
   item 1 is e
   item 2 is l
   item 3 is l
   item 4 is o

// Using a range
for number in 0..5 {
  println!("number is {}", number);
}
=> number is 0
   number is 1
   number is 2
   number is 3
   number is 4
```

### Cases for using each type of loop
- `loop`
  - Repeat a block of code forever
  - Need to return a value
- `while`
  - Continue repeating block of code as long as a confition is true
- `for`
  - Iterate over each item in a collection
  - Repeat a block of code N times -> iterate over range 0..N

### Nested loops example

```
let matrix: [[i32; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];

for row in matrix {
  for num in row {
    print!("{}\t", num);
  }
  println!();
}

=>
  1       2       3
  4       5       6
  7       8       9

// Mutatable version
let mut matrix: [[i32; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];

for row in matrix.iter_mut() {
  for num in row.iter_mut() {
    *num += 10;
    print!("{}\t", num);
  }
  println!();
}
```