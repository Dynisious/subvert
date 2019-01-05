//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/10/4

/// `steal` exists to subvert the Rust compilers borrow checks.
///
/// # Safety
///
/// This macro explicitly invalidates the Rust borrow checks.  
/// You as the programmer are now responsible for ensuring that this reference is dropped
/// before the original.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate subvert; fn main() {
/// use subvert;
/// 
/// let mut a = 0;
///
/// //let b = &mut a; //This line will cause compilation to fail because if you
/// //have a mutable reference you can not have any other references.
///
/// let b = unsafe { steal!(&mut a, i32) }; //This subverts the Rust borrow
///   //checker and allows you to have other references.
///
/// let c: &mut i32 = unsafe { steal!(&mut a) }; //This tries to guess the type of the reference.
///
/// let d = &a;
/// # }
/// ```
#[macro_export]
macro_rules! steal {
    ($ptr:expr; $tp:ty) => (&mut *($ptr as *mut $tp));
    ($ptr:expr) => (steal!($ptr, _));
}
