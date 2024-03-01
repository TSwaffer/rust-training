use std::{boxed, mem, ops::{self, Add}};

fn main() {
  let one = Box::new(1);
  let two = Box::new(2);
  assert_eq!(*sum_boxes(one, two), 3);

  let pi = Box::new(3.14159);
  let e = Box::new(2.71828);
  assert_eq!(*sum_boxes(pi, e), 5.85987);

  println!("Tests passed");
}

fn sum_boxes<T: std::ops::Add<Output = T>>(box1: Box<T>, box2: Box<T>) -> Box<T> {
  let a = *box1;
  let b = *box2;
  let sum = a + b;
  Box::new(sum)
}