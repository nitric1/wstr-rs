# wstr-rs
Rust macros for compile-time UTF-16 (wide) string literals.

## Usage

Add this in your `Cargo.toml` dependencies:
```toml
wstr = "0.1"
```

Then add this in your crate root:
```rust
#[macro_use]
extern crate wstr;
```
If you need `widestr!` macro (for the `WideStr` string), also add a dependency to [`widestring`](http://starkat99.github.io/widestring-rs/widestring/) crate and use `widestring` feature.

Now you are ready to use `wstr!` and `widestr!` macro.
```rust
let wstr = wstr!("Hello, world! \u{1F601}"); // &'static [u16]

extern crate widestring;
let widestr = widestr!("麻雀/麻将 \u{1F007}\u{1F010}\u{1F019}"); // &'static widestring::WideStr
```
