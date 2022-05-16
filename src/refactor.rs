use std::{cell::UnsafeCell, rc::Rc};

pub struct Imp<T> {
    holder: Rc<DerefCell<T>>,
}
impl<T> Imp<T> {
    pub fn new(value: T) -> Self {
        Self {
            holder: Rc::new(DerefCell::new(value)),
        }
    }
    pub fn inner(&self) -> Ref<'_, T> {
        self.holder.get_ref()
    }
    pub fn inner_mut(&mut self) -> RefMut<'_, T> {
        self.holder.get_mut()
    }
}
impl<T> Clone for Imp<T> {
    fn clone(&self) -> Self {
        Self {
            holder: self.holder.clone(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Imp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Imp")
            .field(&*self.holder.get_ref().get())
            .field(&self.holder.mark)
            .finish()
    }
}

///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///

#[derive(Debug)]
enum BorrowMark {
    BorrowMut,
    BorrowImmutable(isize),
    NotBorrowed,
}
struct DerefCell<T> {
    value: UnsafeCell<T>,
    mark: BorrowMark,
}
impl<T> DerefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            mark: BorrowMark::NotBorrowed,
        }
    }

    fn get_ref(&self) -> Ref<'_, T> {
        let mark = match self.mark {
            BorrowMark::BorrowMut => panic!("already mutably borrowed"),
            BorrowMark::BorrowImmutable(i) => BorrowMark::BorrowImmutable(i + 1),
            BorrowMark::NotBorrowed => BorrowMark::BorrowImmutable(1),
        };
        unsafe {
            std::ptr::replace(std::mem::transmute(&self.mark), mark);
            Ref {
                val: &*self.value.get(),
                par: self,
            }
        }
    }

    #[allow(clippy::mut_from_ref)]
    fn get_mut(&self) -> RefMut<'_, T> {
        let mark = match self.mark {
            BorrowMark::BorrowMut => panic!("already mutably borrowed"),
            BorrowMark::BorrowImmutable(_) => panic!("already immutably borrowed"),
            BorrowMark::NotBorrowed => BorrowMark::BorrowMut,
        };
        unsafe {
            std::ptr::replace(std::mem::transmute(&self.mark), mark);
            RefMut {
                val: &mut *self.value.get(),
                par: self,
            }
        }
    }
}

///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///

pub struct Ref<'l, T: 'l> {
    val: &'l T,
    par: &'l DerefCell<T>,
}
impl<'l, T: 'l> Ref<'l, T> {
    pub fn get(&self) -> &T {
        self.val
    }
}
impl<'l, T: 'l> std::ops::Drop for Ref<'l, T> {
    fn drop(&mut self) {
        if let BorrowMark::BorrowImmutable(o) = self.par.mark {
            let mark = if o == 0 {
                BorrowMark::NotBorrowed
            } else {
                BorrowMark::BorrowImmutable(o - 1)
            };
            unsafe {
                std::ptr::replace(std::mem::transmute(&self.par.mark), mark);
            }
        }
    }
}

///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///

pub struct RefMut<'l, T: 'l> {
    val: &'l mut T,
    par: &'l DerefCell<T>,
}
impl<'l, T: 'l> RefMut<'l, T> {
    pub fn get(&mut self) -> &mut T {
        self.val
    }
}
impl<'l, T: 'l> std::ops::Drop for RefMut<'l, T> {
    fn drop(&mut self) {
        if let BorrowMark::BorrowImmutable(o) = self.par.mark {
            let mark = if o == 0 {
                BorrowMark::NotBorrowed
            } else {
                BorrowMark::BorrowImmutable(o - 1)
            };
            unsafe {
                std::ptr::replace(std::mem::transmute(&self.par.mark), mark);
            }
        }
    }
}
