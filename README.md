# Except

The only one `Error`.

**only available in nightly toolchain now.**

## Why?

The official error handling method of Rust is `Result<T, E> where E: Error`.

But Error is too complicated, various types need to be converted, and each crate has its own set of Error.

Even worse, enum nesting will occur, such as:

```rust
enum BazError {
    IO(std::io::Error),
}

enum BarError {
    IO(std::io::Error),
    Baz(BazError),
}

enum FooError {
    IO(std::io::Error),
    Bar(BarError),
}
```

How many times `std::io::Error` occurs here?

The [`anyhow::Error`](https://crates.io/crates/anyhow) is good, but it is generally only used for 
application.

## Solution

*This is just a personal opinion.*

An Error actually only contains the following elements:

- `type`: Auto generated id, used to determine whether the Error is a certain type.
- `sub_type`: Auto generated id, used to determine whether the Error is a certain sub type, used to supplement type.
- `message`: String describing the Error.
- `data`: Optional Error data.
- `backtrace`: Error call stack.
- `source`: Optional previous Error.

For Rust, the `message`, ~~`backtrace`,~~ `source` already exists in `std::error::Error`.

Then I prefer to auto generate the `type`, I think `TypeId` is a solution.

For `data`, I don't have the best idea, because it may be of any type. In order to achieve
only one Error, I chose to use `Box<dyn Any>` internally to save it.

## Example

```rust
use except::ErrorBuilder;

pub struct MyErrorKind;

pub fn foo() -> except::Result<()> {
    Err(ErrorBuilder::new::<MyErrorKind>().message("this is my error").build())
}

pub fn main() {
    if let Err(ex) = foo() {
        if ex.is::<MyErrorKind>() {
            eprintln!("my error detected: {:?}", ex);
        }
    }
}
```

## License

Apache-2.0
