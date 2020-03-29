#   Rust Logic of Action (RLA)
Leslie Lamport developed a logic he called Temporal Logic of Actions to describe behaviours of concurrent systems.
TLA+ is a formal specification language to design, model, document, and verify programs, especially concurrent systems and distributed systems.

##  The Idea
I suggest an rusty implementation and syntax to formally describe the programme and automatically check

0.  if this specification is flawed and
1.  if the code of the programme implements this specification.

Do do so, a language have to be defined:

### Initial State

TLA+ uses `init` and `next`.
I prefer to use `let` and `let mut` to declare the constants and variables and assign the initial values:

```rust
  let a_constant_value: &'static str = "constant";
  let mut variable = { 23, 42 };
  let mut a_range = 0..1337
```

### Operators

The Operators are the same as in Rust, which is quite different from TLA+:

* addition, subtraction, multiplication, division, modulo: `+`, `-`, `*`, `/`, `%`
* equal, unequal, less than, greater than, less or equal, greater or equal: `==`, `!=`, `<`, `>`, `<=`, `>=`
* logical and, or, not: `&&`, `||`, `!`

### For-All and Exist

There are predefined generic functions `forAll` and `exists`.
