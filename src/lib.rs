#![feature(dispatch_from_dyn)]
#![feature(unsize)]
#![feature(coerce_unsized)]

mod imp_impls;
mod tests;

use std::{cell::RefCell, rc::Rc};

#[doc = include_str!("../readme.md")]
pub struct Imp<T: ?Sized> {
    v: Rc<RefCell<T>>,
}

#[allow(unused)]
impl<T> Imp<T> {
    /// Returns a pointer to the data
    ///
    /// # Arguments
    ///
    /// * `t` - The value to be pointed to.
    ///
    /// # Examples
    ///
    /// ```
    /// use interior_mutability_pointer::Imp;
    /// let p = Imp::new(String::new());
    /// let p2 = p.clone();
    /// p.push_str("yoo"); // Modifies the inner value of both p and p2.
    /// ```
    pub fn new(t: T) -> Self {
        Self {
            v: Rc::new(RefCell::new(t)),
        }
    }

    /// Returns true if two pointers are equal
    ///
    /// # Arguments
    /// * `this` - A pointer to compare
    /// * `other` - The other pointer to compare to
    ///
    /// # Examples
    /// ```
    /// use interior_mutability_pointer::Imp;
    /// let p1 = Imp::new(String::new());
    /// let p2 = p1.clone();
    /// let p3 = Imp::new(String::new());
    /// println!("{}", Imp::ptr_eq(&p1, &p2)); // Prints true
    /// println!("{}", Imp::ptr_eq(&p1, &p3)); // Prints false
    /// ```
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        Rc::ptr_eq(&this.v, &other.v)
    }
}

/*
    Implements cloning the pointer.
*/
mod clone_impl {
    use super::Imp;
    use std::clone::Clone;

    impl<T> Clone for Imp<T> {
        fn clone(&self) -> Self {
            Self { v: self.v.clone() }
        }
    }
}

/*
    Allows access to the inner methods from T.
*/
mod deref_impl {
    use std::{
        marker::Unsize,
        ops::{CoerceUnsized, Deref, DerefMut, DispatchFromDyn},
    };

    use super::Imp;

    impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Imp<U>> for Imp<T> {}
    impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Imp<U>> for Imp<T> {}

    impl<T: ?Sized> Deref for Imp<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.v.as_ptr() }
        }
    }

    impl<T: ?Sized> DerefMut for Imp<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.v.as_ptr() }
        }
    }
}
