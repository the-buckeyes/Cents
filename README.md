# cents

A small crate for handling monetary values as in integer number of cents.

### Example Usage

```rust
use cents::Cents;

// Create a new cents object directly.
let money = Cents::new(21, 21);

// compare it against a cents object created from a u64 value.
assert_eq!(Cents::new(42, 42), 4242.into());


// add it to an u64 value.
assert_eq!(money + 2121, Cents::new(42, 42);

// Add two cents
assert_eq!(Cents::from(2121) + Cents::from(2121), Cents::new(42, 42));
```

### Features

- Addition (Self, u64)
- Subtraction (Self)
- Multiplication (Self, u64)
- Conversion from BigInt
- Conversion from u64
- Equality (Self)
