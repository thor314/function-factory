# What this
This is a play repo I spun up to test my knowledge of building a generic
function factories in Rust. Doing so is a great exercise! It requires knowledge
of:
- higher kinded functions, eg `greeter: impl Fn(&'a T, &'a T, &'a str) ->
  Box<dyn Fn() -> String>`
- Trait objects (see dyn Fn.. above)
- Opaque types (see impl Fn.. adove)
- Lifetimes, if passing by reference in the function factory
- Closures that `move` data into their scope

and idk, probably more, in 63 lines (many of which are comments).

## A good two tests to see if you understand this repo
- What should this argument type signature do, and why is it broken?
```rust
greeter: impl Fn(&'a T, &'a T, &'a str) -> impl Fn() -> String`
```
- On line 8, what does `move` do? 
```rust
    Box::new(move || format!("{} {}, {}", greeting, name, comment))
```

