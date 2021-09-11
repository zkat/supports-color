Detects whether a terminal supports color, and gives details about that
support. It takes into account the `COLOR` and `NO_COLOR` environment
variables.

This crate is a Rust port of [@sindresorhus](https://github.com/sindresorhus)'
[NPM package by the same name](https://npm.im/supports-color).

## Example

```rust
use supports_color::Stream;

let support = supports_color::on(Stream::stdout);
if support.has_16m {
    println!("16 million (RGB) colors are supported");
} else if support.has_256 {
    println!("256-bit colors are supported.");
} else if support.has_basic {
    println!("Only basic ANSI colors are supported.");
} else {
    println!("No color support.");
}
```
