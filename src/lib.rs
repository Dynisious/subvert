//! `subvert` is a crate meant to provide common utilities which are `unsafe` according
//! to the Rust compiler but sometimes prove to be a necessary evil to make our code more
//! efficient or to keep our solutions simple.
//!
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2017/12/4

#![warn(missing_docs)]

/// `steal` exists to subvert the Rust compilers borrow checks.
///
/// #Safety
///
/// This function explicitly invalidates the Rust borrow checks.  
/// You as the programmer are now responsible for ensuring that this reference is dropped
/// before the original.
///
/// #Examples
///
/// ```
/// # #[macro_use] extern crate subvert; fn main() {
///     let mut a = 0;
///
///     //let b = &mut a; //This line will cause compilation to fail because if you
///         //have a mutable reference you can not have any other references.
///
///     let b: &mut _ = unsafe { steal!(&mut a) }; //This subverts the Rust borrow
///         //checker and allows you to have other references.
///
///     let c = &a;
/// # }
/// ```
#[macro_export]
macro_rules! steal {
    ($ptr:expr) => (&mut *($ptr as *mut _))
}
