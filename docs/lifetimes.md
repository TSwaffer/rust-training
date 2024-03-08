# Lifetimes

## Lifetime annotation
- Explicitly defines a generic lifteime for parameters
- Must begin with an apostrophe (`'`) symbol
- Names are conventionally single lowercase letters

```
fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```

## Liftime elision rules
- Set of rules for the compiler to analyse reference lifetimes
- Describes situations that do not require explicit lifteime annotations
- If any ambiguity remains, explicit annotation will be requried
- There are three rules

### Rule 1
Each input parameter that is a reference is assigned its own lifetime.

### Rule 2
If there is exactly one input lifetime, assign it to all output lifetimes.

### Rule 3
If there is a `&self` or `&mut self` input parameter, its lifetime will be assigned to all output parameters.