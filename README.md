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

- `kind`: Auto generated id, used to determine whether the Error(Exception) is a certain type.
- `code`: For business logic used or sub type detected.
- `message`: String describing the Error(Exception).
- `data`: Optional Error(Exception) data.
- `backtrace`: Error(Exception) call stack.
- `source`: Optional previous Error(Exception).

For Rust, the `message`, ~~`backtrace`,~~ `source` already exists in `std::error::Error`.

Then I prefer to auto generate the `code(kind)`, I think `TypeId` is a solution.

For `data`, I don't have the best idea, because it may be of any type. In order to achieve
only one Error(Exception), I chose to use `Box<dyn Any>` internally to save it.

## Example

I use the name `Exception` because it is less commonly used in Rust.

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

Image you want to implement the http api response json.

```rust
use except::Exception;

pub trait Serialize {
    fn serialize(&self) -> String;
}

impl Serialize for () {
    fn serialize(&self) -> String {
        "null".to_string()
    }
}

#[repr(u32)]
#[non_exhaustive]
pub enum ApiError {
    Unauthorized = 401,
    Forbidden = 403,
    InternalServerError = 500,
}

pub struct ApiResponse {
    code: u32,
    message: String,
    data: Box<dyn Serialize>,
}

pub fn something_failed() -> Result<(), Exception> {
    Err(Exception::new1::<ApiError>("login required", ApiError::Unauthorized as u32))
}

pub fn handle_api_exception(ex: Exception) -> ApiResponse {
    if ex.is::<ApiError>() {
        ApiResponse {
            code: ex.get_code(),
            message: ex.to_string(),
            data: ex.into_data::<Box<dyn Serialize>>().unwrap(),
        }
    } else {
        ApiResponse {
            code: 500,
            message: ex.to_string(),
            data: Box::new(()),
        }
    }
}
```

## License

Apache-2.0
