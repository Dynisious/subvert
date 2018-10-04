//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/10/4

use std::ops;

/// Allows double indexing into a type.
pub trait Index2<Index: Eq>: ops::IndexMut<Index> {
  /// Double indexes into a type.
  /// 
  /// # Params
  /// 
  /// fst --- The first index.  
  /// snd --- The second index.
  /// 
  /// # Example
  /// 
  /// ```
  /// use subvert::Index2;
  /// 
  /// assert_eq!(vec![2, 1,].index2(0, 1,), (&mut 2, &mut 1,),);
  /// ```
  #[inline]
  fn index2(&mut self, fst: Index, snd: Index,) -> (&mut Self::Output, &mut Self::Output,) {
    assert!(fst != snd, "indexes cannot be the same");

    (unsafe { &mut *(&mut self[fst] as *mut _) }, &mut self[snd],)
  }
}

impl<T,> Index2<usize> for [T] {}
