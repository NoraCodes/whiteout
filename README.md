# whiteout

[![crates.io badge](https://img.shields.io/crates/v/whiteout.svg)](https://crates.io/crates/whiteout) [documentation](https://docs.rs/whiteout/)  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 

`whiteout` provides macros that erase the type of any value into an `impl Trait` for a given trait. This gives you a value that can only be use with the methods of that trait, and whose type is unnameable.

```rust
#[macro_use] extern crate whiteout;
fn main() {
    let a = erase!(10, std:ops::Add<i64, Output=i64>);
    assert_eq!(a + 10, 20);
}
```
Since we sometimes want to use these values together, `whiteout` provides both a one-time macro and a macro that produces a function which returns a consistent unnameable type, allowing multiple erased values to be used in conjunction. See the documentation for more info.

