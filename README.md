# Except

For Rust, a better `try/catch`-like error handler rather than `result`.

# Why

The official error handler for unwind-able error is `Result`, but why there is `panic`? Because `panic` is generally should not be handle,
but why there is `catch-unwind`, because sometimes `panic` must be treaded. So why not merge them to one way, just like `Java` and `PHP`,
to a better `throw` and `try-catch` process.

## Not good of Result
- Merge business logic and error handle logic, but now there is `?`, hack-ful but can solve this problem.
- No backtrace, it is painful for we to debug and find the bug reason of the program, there is [fix-error rfc](https://github.com/rust-lang/rfcs/blob/master/text/2504-fix-error.md) or `backtrace` crates,
  but take time to implement.
- No unified struct type of `Error`, it is painful for me to handle a lot of Type cast, otherwise there is `failure` and `error-chain` crates.
- The worse, you can't return a error type not suite the function Type declaration, like checked-exception. This is sometimes make us crash,
  so we have to use `unwrap` or `except` to handle error, this is not runtime-safe, and also forced obsessive-compulsive disorder.

# Roadmap

Rather than `Result` or `checked exception` or `dynamic exception`, I will implement a compile-time auto-generated enum Type for every catch, just like 
closure do, for better `try-catch`.

