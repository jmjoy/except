# Except

Exception: The only `Error` you deserve.

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

An Error(Exception) actually only contains the following elements:

- `code(kind)`: Used to determine whether two Errors(Exceptions) are of the same type.
- `message`: String describing the Error(Exception).
- `data`: Optional Error(Exception) data.
- `backtrace`: Error(Exception) call stack.
- `source`: Optional previous Error(Exception).

For Rust, the `message`, `backtrace`, `source` already exists in `std::error::Error`.

Then I prefer to auto generate the `code(kind)`, I think `TypeId` is a solution.

For `data`, I don't have the best idea, because it may be of any type. In order to achieve
only one Error(Exception), I chose to use `Box<dyn Any>` internally to save it.

## Example

I use the name `Exception` here because it is less commonly used in Rust.

```rust
use except::Exception;

pub struct MyErrorKind;

pub fn foo() -> Result<(), Exception> {
    Err(Exception::new::<MyErrorKind>("this is my error"))
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
