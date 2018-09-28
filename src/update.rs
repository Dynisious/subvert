
use std::ptr;

/// Updates the value behind the passed reference using a byval update function.
/// 
/// There are many cases when the compiler will complain that you are moving out of a
/// borrowed value because it cannot ensure that you will repopulate the value. Using
/// this function guarrenties that you will, making it a safe operation.
/// 
/// # Params
/// 
/// val --- The mutable reference to the value to update.  
/// func --- The update function to use.
/// 
/// # Example
/// 
/// ```
/// use subvert::update;
/// 
/// struct Foo(pub u32,);
/// 
/// let mut x = Foo(10,);
/// let x = &mut x;
/// 
/// //Error: cannot move out of borrowed content
/// // *x = match *x {
/// //     x => Foo(x.0 * 10,),
/// // };
/// 
/// update(x, |x: Foo| Foo(x.0 * 10,));
/// 
/// assert_eq!(100, x.0,);
/// ```
#[cfg(not(pin))]
pub fn update<T, I, O,>(val: &mut T, func: impl FnOnce(I,) -> O,)
    where T: Into<I> + From<O>, {
    unsafe { ptr::write(val, func(ptr::read(val).into()).into(),) }
}
#[cfg(pin)]
pub fn update<T, I, O,>(val: &mut T, func: impl FnOnce(I,) -> O,)
    where T: ::std::pin::Unpin + Into<I> + From<O>, {
    unsafe { ptr::write(val, func(ptr::read(val).into()).into(),) }
}
