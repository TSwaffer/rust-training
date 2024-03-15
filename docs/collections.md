# Collections

## Vectors data type

`Vec<T>`

Holds a collection of elements with the same data type with the elements stored in order. It is different form an error as items can be dynamically added and removed.

```
fn main() {
  let mut vec = Vec<Sting> = Vec::new();
  vec.push(String::from("a"));
  vec.push(String::from("b"));
  ...

  let second = &vec[1];

  // Simpler way to initialise
  let numbers = vec![1,2,3,4,5];
}
```

## HashMap<K,V> data type

- Stores data in key -> value pairs
- Keys and values can be different types, but all keys and all values must have the same data type
- Each key can only have one value associated with it
- You should not rely on keys being stored in the order they are inserted

```
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert("a", 1);
  map.insert("b", 2);
  map.insert("c", 3);

  let a_value = map.get("a");
}
```

### Updating HashMap entries
There are three common ways to update an entry in a HashMap:

1. Overwrite an existing key-value pair
2. Insert a new entry if a key does not exist
3. Modify a value based on its existing value

```
map.insert("a", 4);

map.entry("d").or_insert(2);

let c = map.entry("c").or_insert(0);
*c += 1;
```