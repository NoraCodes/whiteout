//! `whiteout` provides macros that erase the type of any value into 
//! an impl Trait for a given trait. Obviously, this requires 
//! `#![feature(conservative_impl_trait)]` to be enabled on your
//! crate root.
//!
//! # Examples
//!
//! Single values can be erased using the `erase!` macro.
//!
//! ```
//! #![feature(conservative_impl_trait)]
//! #[macro_use]
//! extern crate whiteout;
//! fn main() {
//!     let a = erase!(10, std::ops::Add<i64, Output=i64>);
//!     assert_eq!(a + 10, 20);
//! }
//! ```
//!
//! These erased values can't be used together, though; they have different
//! anonymized types. Therefore, you sometimes need the `eraser!` macro.
//!
//! ```
//! #![feature(conservative_impl_trait)]
//! #[macro_use]
//! extern crate whiteout;
//!
//! // Define a custom trait and a blanket impl.
//! trait MyTrait: 
//!     std::ops::Add<Self, Output=Self> 
//!     + std::convert::From<i32> 
//!     + std::fmt::Debug 
//!     + PartialEq 
//!     {}
//!
//! impl<T> MyTrait for T 
//!     where T: std::ops::Add<Self, Output=Self> 
//!     + std::convert::From<i32> 
//!     + std::fmt::Debug
//!     + PartialEq
//!     {}
//!
//! // Create an eraser function for our custom trait
//! eraser!(erase_my_trait, MyTrait);
//!
//! fn main() {
//!     // Use the eraser function. 
//!     // If we used erase!(10, MyTrait); for these
//!     // they would be of different types.
//!     let a = erase_my_trait(10);
//!     let b = erase_my_trait(10);
//!     assert_eq!(a + b, 20.into());
//! }
//! ```
//!
//!

#![feature(conservative_impl_trait)]

/// `eraser!(name, trait)` creates a function with the given identifier that 
/// erases values to an anonymous type that is `impl Trait` for the given trait
#[macro_export]
macro_rules! eraser {
    ($name:ident, $($tr:tt)*) => {
            // This function takes any type implementing T and returns impl T
            fn $name<T: $($tr)*>(val: T) -> impl $($tr)* {
                // Do nothing to the value
                val
            }
    }
}

/// `erase!(value, trait)` turns a value of any type that implements trait into 
/// an erasted type which is `impl Trait` for that trait.
#[macro_export]
macro_rules! erase {
    ($val:expr, $($tr:tt)*) => {
        // Creates a block to operate in
        {
            eraser!(f, $($tr)*);
            // Immediately use this function
            f($val)
        }
    }
}

