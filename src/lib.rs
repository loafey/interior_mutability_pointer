mod imp_impls;
mod tests;

use std::{cell::RefCell, rc::Rc};

/// A wrapper around `Rc<RefCell<T>>` allowing immediate access to inner methods,
/// without the need for `.borrow()` or `.borrow_mut()`,
/// allowing for a more seamless pointer experience.
/// ```
/// let mut k = Imp::new(String::new());
/// let p = k.clone(); // Clone the pointer.
/// k.push_str("yo");
/// println!("{} {}", k, p); // Prints "yo yo"
/// ```
///
/// Also allows the use of operators:
/// ```
/// let mut k = Imp::new(5);
/// let p = k.clone(); // Clone the pointer.
/// k += 5;
/// println!("{} {}", k, p); // Prints "10 10"
/// ```
///
/// The biggest difference to `Rc<RefCell<T>>` is that your pointer instance will need to be marked as `mut`
/// if you want to use `&mut self` methods, as opposed to `Rc<RefCell<T>>` instances where you can call `.borrow_mut()`,
/// removing the need for the mut keyword.
///
/// However, this does not mean that all clones of the pointer need to be mutable!
/// ```
/// let k = Imp::new(String::new());
/// let mut p = k.clone(); // Clone the pointer.
/// p.push_str("yo");
/// println!("{:?} {:?}", k, p); // Prints "yo yo"
/// ```

pub struct Imp<T> {
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
    use std::ops::{Deref, DerefMut};

    use super::Imp;

    impl<T> Deref for Imp<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { self.v.try_borrow_unguarded().unwrap() }
        }
    }

    impl<T> DerefMut for Imp<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.v.as_ptr() }
        }
    }
}
