//! `subvert` is a crate meant to provide common utilities which are `unsafe` according
//! to the Rust compiler but:
//! * Are common actions which cannot be tracked by the compiler.
//! * Sometimes prove to be a necessary evil.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/10/2

#![deny(missing_docs)]

#![cfg_attr(feature = "pin", feature(pin))]

mod steal;
mod update;
mod index;

pub use self::steal::*;
pub use self::update::*;
pub use self::index::*;
