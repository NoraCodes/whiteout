# whiteout

[![crates.io badge](https://img.shields.io/crates/v/whiteout.svg)](https://crates.io/crates/whiteout) [documentation](https://docs.rs/whiteout/)  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 

`whiteout` provides macros that erase the type of any value into an `impl Trait` for a given trait. Obviously, this requires `#![feature(conservative_impl_trait)]` to be enabled on your crate root.
