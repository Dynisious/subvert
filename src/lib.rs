//! `subvert` is a crate meant to provide common utilities which are `unsafe` according
//! to the Rust compiler but sometimes prove to be a necessary evil to make our code more
//! efficient or to keep our solutions simple.
//!
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2017/12/4

#![deny(missing_docs)]

#![cfg_attr(feature = "pin", feature(pin))]

mod steal;
mod update;

pub use self::steal::*;
pub use self::update::*;
