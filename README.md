# Ed

_Ed, man! !man ed_

As we all know, `ed` is the standard text editor, and is also the path to redemption. Unfortunately, it is currently written in C, which is an unsafe language. This project is an effort to rewrite the glorious `ed` editor in none other than Rust.

## QuickStart

Assuming that you have Rust 1.72.0 or later installed that targets your native target triple.

```
cargo run --release
```

The commands are the same as in normal `ed`. To open a file.

```
cargo run --release -- filename
```
