//! `whiteout` provides macros that erase the type of any value into 
//! an `impl Trait` for a given trait.
//!
//! # Examples
//!
//! Single values can be erased using the `erase!` macro.
//!
//! ```
//!# #[macro_use]
//!# extern crate whiteout;
//!# fn main() {
//! let a = erase!(10, std::ops::Add<i64, Output=i64>);
//! let b = erase!(5, std::ops::Add<i64, Output=i64>);
//! assert_eq!(a + 10, 20);
//! assert_eq!(b + 10, 15);
//! // This fails, the types are opaque
//! // assert_eq!(a + b, 15);
//!# }
//! ```
//!
//! These erased values can't be used together, though; they have different
//! anonymized types. Therefore, you sometimes need the `eraser!` macro.
//!
//! ```
//! #[macro_use]
//! extern crate whiteout;
//!
//! // Define a custom trait into which types will be erased.
//! trait MyTrait: 
//!     std::ops::Add<Self, Output=Self>  // Allow the operation we need
//!     + std::convert::From<i32>  // Allow converting from concrete values
//!     + std::fmt::Debug  // Allow printing (for use with assert!())
//!     + PartialEq  // Allow comparison (for use with assert_eq!())
//!     {}
//!
//! // Implement MyTrait for all possible types.
//! impl<T> MyTrait for T 
//!     where T: std::ops::Add<Self, Output=Self> 
//!     + std::convert::From<i32> 
//!     + std::fmt::Debug
//!     + PartialEq
//!     {}
//!
//! // Create an eraser function for the custom trait
//! eraser!(erase_my_trait, MyTrait);
//!
//! fn main() {
//!     // Use the eraser function. 
//!     // If we used erase!(10, MyTrait); for these
//!     // they would be of different types.
//!     let a = erase_my_trait(10);
//!     let b = erase_my_trait(5);
//!     assert_eq!(a + b, 15.into());
//! }
//! ```
//!
//!


/// `eraser!(name, trait)` creates a function with the given identifier that 
/// erases values to an anonymous type that is `impl Trait` for the given trait.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate whiteout;
///
/// // Define a custom trait into which types will be erased.
/// trait MyTrait: 
///     std::ops::Add<Self, Output=Self>  // Allow the operation we need
///     + std::convert::From<i32>  // Allow converting from concrete values
///     + std::fmt::Debug  // Allow printing (for use with assert!())
///     + PartialEq  // Allow comparison (for use with assert_eq!())
///     {}
///
/// // Implement MyTrait for all possible types.
/// impl<T> MyTrait for T 
///     where T: std::ops::Add<Self, Output=Self> 
///     + std::convert::From<i32> 
///     + std::fmt::Debug
///     + PartialEq
///     {}
///
/// // Create an eraser function for the custom trait
/// eraser!(erase_my_trait, MyTrait);
///
/// fn main() {
///     // Use the eraser function. 
///     // If we used erase!(10, MyTrait); for these
///     // they would be of different types.
///     let a = erase_my_trait(10);
///     let b = erase_my_trait(5);
///     assert_eq!(a + b, 15.into());
/// }
/// ```
///
///
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
/// 
/// # Examples
///
///
/// ```
///# #[macro_use]
///# extern crate whiteout;
///# fn main() {
/// let a = erase!(10, std::ops::Add<i64, Output=i64>);
/// let b = erase!(5, std::ops::Add<i64, Output=i64>);
/// assert_eq!(a + 10, 20);
/// assert_eq!(b + 10, 15);
/// // This fails, the types are opaque
/// // assert_eq!(a + b, 15);
///# }
/// ```
///
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

