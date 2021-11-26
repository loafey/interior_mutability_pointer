mod imp_impls;
mod tests;

use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Imp<T> {
    v: Rc<RefCell<T>>,
}

#[allow(unused)]
impl<T> Imp<T> {
    pub fn new(t: T) -> Self {
        Self {
            v: Rc::new(RefCell::new(t)),
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
