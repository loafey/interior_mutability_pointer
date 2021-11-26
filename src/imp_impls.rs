/*
    Allows using != and ==.
*/
mod eq_partial_eq_impl {
    use std::ops::Deref;

    use crate::Imp;

    impl<T: PartialEq> PartialEq for Imp<T> {
        fn eq(&self, other: &Self) -> bool {
            *self.deref() == *other.deref()
        }
    }

    impl<T: PartialEq> PartialEq<T> for Imp<T> {
        fn eq(&self, other: &T) -> bool {
            let a = self.deref();
            let b = other.deref();
            a.eq(b)
        }
    }
    impl<T: Eq> Eq for Imp<T> {}
}

/*
    Allows using < > >= <=
*/
mod partialord_ord_impl {
    use crate::Imp;
    use std::ops::Deref;

    impl<T: PartialOrd> PartialOrd for Imp<T> {
        fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
            let a = self.deref();
            let b = other.deref();
            if a < b {
                Some(std::cmp::Ordering::Less)
            } else if a > b {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
    impl<T: PartialOrd> PartialOrd<T> for Imp<T> {
        fn partial_cmp(&self, other: &T) -> std::option::Option<std::cmp::Ordering> {
            let a = self.deref();
            let b = other;
            if a < b {
                Some(std::cmp::Ordering::Less)
            } else if a > b {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
    impl<T: Ord> Ord for Imp<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}
/*
    Allows using the Debug and Display implementation of the inner T.
*/
mod debug_display_impl {
    use std::fmt::Debug;
    use std::fmt::Display;

    use crate::Imp;

    impl<T: Debug> Debug for Imp<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.v.borrow().fmt(f)
        }
    }

    impl<T: Display> Display for Imp<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
            self.v.borrow().fmt(f)
        }
    }
}

/*
    Allows indexing with range bounds.
*/
mod index_indexmut_impl {
    use std::ops::{Deref, DerefMut, Index, IndexMut, Range};

    use crate::Imp;

    impl<T: Index<usize>> Index<usize> for Imp<T> {
        type Output = T::Output;

        fn index(&self, index: usize) -> &Self::Output {
            let p = self.deref();
            &p[index]
        }
    }
    impl<T: Index<Range<usize>>> Index<Range<usize>> for Imp<T> {
        type Output = T::Output;

        fn index(&self, index: Range<usize>) -> &Self::Output {
            let p = self.deref();
            &p[index]
        }
    }
    impl<T: IndexMut<usize>> IndexMut<usize> for Imp<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            let p = self.deref_mut();
            p.index_mut(index)
        }
    }
    impl<T: IndexMut<Range<usize>>> IndexMut<Range<usize>> for Imp<T> {
        fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
            let p = self.deref_mut();
            p.index_mut(index)
        }
    }
}

/*
    Allows the use of the + operator
*/
mod add_impl {
    use crate::Imp;
    use std::ops::{Add, Deref};

    impl<T: Add<T> + Copy + Add<Output = T>> Add for Imp<T> {
        type Output = Imp<T>;

        fn add(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.add(b);
            Imp::new(r)
        }
    }
    impl<T: Add<T> + Copy + Add<Output = T>> Add<T> for Imp<T> {
        type Output = Imp<T>;

        fn add(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.add(b);
            Imp::new(r)
        }
    }
}
/*
    Allows the use of the += operator
*/
mod add_assign_impl {
    use crate::Imp;
    use std::ops::AddAssign;

    impl<T: AddAssign<T> + Copy + AddAssign> AddAssign for Imp<T> {
        fn add_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().add_assign(*other.v.borrow());
        }
    }
    impl<T: AddAssign<T> + Copy + AddAssign> AddAssign<T> for Imp<T> {
        fn add_assign(&mut self, other: T) {
            self.v.borrow_mut().add_assign(other);
        }
    }
}

/*
    Allows the use of the & operator
*/
mod bitand_impl {
    use crate::Imp;
    use std::ops::{BitAnd, Deref};

    impl<T: BitAnd<T> + Copy + BitAnd<Output = T>> BitAnd for Imp<T> {
        type Output = Self;

        fn bitand(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.bitand(b);
            Imp::new(r)
        }
    }
    impl<T: BitAnd<T> + Copy + BitAnd<Output = T>> BitAnd<T> for Imp<T> {
        type Output = Self;

        fn bitand(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.bitand(b);
            Imp::new(r)
        }
    }
}

/*
    Allows the use of the &= operator.
    BitAndAssign    The bitwise AND assignment operator &=.
*/
mod bitand_assign_impl {
    use crate::Imp;
    use std::ops::BitAndAssign;

    impl<T: BitAndAssign<T> + Copy + BitAndAssign> BitAndAssign for Imp<T> {
        fn bitand_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().bitand_assign(*other.v.borrow());
        }
    }
    impl<T: BitAndAssign<T> + Copy + BitAndAssign> BitAndAssign<T> for Imp<T> {
        fn bitand_assign(&mut self, other: T) {
            self.v.borrow_mut().bitand_assign(other);
        }
    }
}

/*
    Allows the use of the | operator
*/
mod bitor_impl {
    use crate::Imp;
    use std::ops::{BitOr, Deref};

    impl<T: BitOr<T> + Copy + BitOr<Output = T>> BitOr for Imp<T> {
        type Output = Self;

        fn bitor(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.bitor(b);
            Imp::new(r)
        }
    }
    impl<T: BitOr<T> + Copy + BitOr<Output = T>> BitOr<T> for Imp<T> {
        type Output = Self;

        fn bitor(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.bitor(b);
            Imp::new(r)
        }
    }
}

/*
    Allows the use of the |= operator
*/
mod bitor_assign_impl {
    use crate::Imp;
    use std::ops::BitOrAssign;

    impl<T: BitOrAssign<T> + Copy + BitOrAssign> BitOrAssign for Imp<T> {
        fn bitor_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().bitor_assign(*other.v.borrow());
        }
    }
    impl<T: BitOrAssign<T> + Copy + BitOrAssign> BitOrAssign<T> for Imp<T> {
        fn bitor_assign(&mut self, other: T) {
            self.v.borrow_mut().bitor_assign(other);
        }
    }
}

/*
    Allows the use of the ^ operator
*/
mod bitxor_impl {
    use crate::Imp;
    use std::ops::{BitXor, Deref};

    impl<T: BitXor<T> + Copy + BitXor<Output = T>> BitXor for Imp<T> {
        type Output = Self;

        fn bitxor(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.bitxor(b);
            Imp::new(r)
        }
    }
    impl<T: BitXor<T> + Copy + BitXor<Output = T>> BitXor<T> for Imp<T> {
        type Output = Self;

        fn bitxor(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.bitxor(b);
            Imp::new(r)
        }
    }
}

/*
    Allows the use of the ^= operator
*/
mod bitxor_assign_impl {
    use crate::Imp;
    use std::ops::BitXorAssign;

    impl<T: BitXorAssign<T> + Copy + BitXorAssign> BitXorAssign for Imp<T> {
        fn bitxor_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().bitxor_assign(*other.v.borrow());
        }
    }
    impl<T: BitXorAssign<T> + Copy + BitXorAssign> BitXorAssign<T> for Imp<T> {
        fn bitxor_assign(&mut self, other: T) {
            self.v.borrow_mut().bitxor_assign(other);
        }
    }
}

/*
    Allows the use of the ! operator
*/
mod not_impl {
    use crate::Imp;
    use std::ops::Not;

    impl<T: Not + Copy> Not for Imp<T> {
        type Output = Imp<T::Output>;

        fn not(self) -> Self::Output {
            Imp::new(self.v.borrow().not())
        }
    }
}

/*
    Allows the use of the / operator
*/
mod div_impl {
    use crate::Imp;
    use std::ops::{Deref, Div};

    impl<T: Div<T> + Copy + Div<Output = T>> Div for Imp<T> {
        type Output = Self;

        fn div(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.div(b);
            Imp::new(r)
        }
    }
    impl<T: Div<T> + Copy + Div<Output = T>> Div<T> for Imp<T> {
        type Output = Self;

        fn div(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.div(b);
            Imp::new(r)
        }
    }
}

/*
    Allows the use of the /= operator
*/
mod div_assign_impl {
    use crate::Imp;
    use std::ops::DivAssign;

    impl<T: DivAssign<T> + Copy + DivAssign> DivAssign for Imp<T> {
        fn div_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().div_assign(*other.v.borrow());
        }
    }
    impl<T: DivAssign<T> + Copy + DivAssign> DivAssign<T> for Imp<T> {
        fn div_assign(&mut self, other: T) {
            self.v.borrow_mut().div_assign(other);
        }
    }
}

/*
    Allows the use of the * operator
*/
mod mul_impl {
    use crate::Imp;
    use std::ops::{Deref, Mul};

    impl<T: Mul<T> + Copy + Mul<Output = T>> Mul for Imp<T> {
        type Output = Self;

        fn mul(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.mul(b);
            Imp::new(r)
        }
    }
    impl<T: Mul<T> + Copy + Mul<Output = T>> Mul<T> for Imp<T> {
        type Output = Self;

        fn mul(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.mul(b);
            Imp::new(r)
        }
    }
}

/*
    Allows the use of the *= operator
*/
mod mul_assign_impl {
    use crate::Imp;
    use std::ops::MulAssign;

    impl<T: MulAssign<T> + Copy + MulAssign> MulAssign for Imp<T> {
        fn mul_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().mul_assign(*other.v.borrow());
        }
    }
    impl<T: MulAssign<T> + Copy + MulAssign> MulAssign<T> for Imp<T> {
        fn mul_assign(&mut self, other: T) {
            self.v.borrow_mut().mul_assign(other);
        }
    }
}

/*
    Allows the use of the ! operator
*/
mod neg_impl {
    use crate::Imp;
    use std::ops::Neg;

    impl<T: Neg + Copy> Neg for Imp<T> {
        type Output = Imp<T::Output>;

        fn neg(self) -> Self::Output {
            Imp::new(self.v.borrow().neg())
        }
    }
}

/*
    Allows the use of the % operator
*/
mod rem_impl {
    use crate::Imp;
    use std::ops::{Deref, Rem};

    impl<T: Rem<T> + Copy + Rem<Output = T>> Rem for Imp<T> {
        type Output = Self;

        fn rem(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.rem(b);
            Imp::new(r)
        }
    }
    impl<T: Rem<T> + Copy + Rem<Output = T>> Rem<T> for Imp<T> {
        type Output = Self;

        fn rem(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.rem(b);
            Imp::new(r)
        }
    }
}

/*
    Allows the use of the %= operator
*/
mod rem_assign_impl {
    use crate::Imp;
    use std::ops::RemAssign;

    impl<T: RemAssign<T> + Copy + RemAssign> RemAssign for Imp<T> {
        fn rem_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().rem_assign(*other.v.borrow());
        }
    }
    impl<T: RemAssign<T> + Copy + RemAssign> RemAssign<T> for Imp<T> {
        fn rem_assign(&mut self, other: T) {
            self.v.borrow_mut().rem_assign(other);
        }
    }
}

/*
    Allows the use of the << operator
*/
mod shl_impl {
    use crate::Imp;
    use std::ops::{Deref, Shl};

    impl<T: Shl<T> + Copy + Shl<Output = T>> Shl for Imp<T> {
        type Output = Self;

        fn shl(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.shl(b);
            Imp::new(r)
        }
    }
    impl<T: Shl<T> + Copy + Shl<Output = T>> Shl<T> for Imp<T> {
        type Output = Self;

        fn shl(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.shl(b);
            Imp::new(r)
        }
    }
}

/*
    Allows the use of the <<= operator
*/
mod shl_assign_impl {
    use crate::Imp;
    use std::ops::ShlAssign;

    impl<T: ShlAssign<T> + Copy + ShlAssign> ShlAssign for Imp<T> {
        fn shl_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().shl_assign(*other.v.borrow());
        }
    }
    impl<T: ShlAssign<T> + Copy + ShlAssign> ShlAssign<T> for Imp<T> {
        fn shl_assign(&mut self, other: T) {
            self.v.borrow_mut().shl_assign(other);
        }
    }
}

/*
    Allows the use of the >> operator
*/
mod shr_impl {
    use crate::Imp;
    use std::ops::{Deref, Shr};

    impl<T: Shr<T> + Copy + Shr<Output = T>> Shr for Imp<T> {
        type Output = Self;

        fn shr(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.shr(b);
            Imp::new(r)
        }
    }
    impl<T: Shr<T> + Copy + Shr<Output = T>> Shr<T> for Imp<T> {
        type Output = Self;

        fn shr(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.shr(b);
            Imp::new(r)
        }
    }
}

/*
    Allows the use of the >>= operator
*/
mod shr_assign_impl {
    use crate::Imp;
    use std::ops::ShrAssign;

    impl<T: ShrAssign<T> + Copy + ShrAssign> ShrAssign for Imp<T> {
        fn shr_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().shr_assign(*other.v.borrow());
        }
    }
    impl<T: ShrAssign<T> + Copy + ShrAssign> ShrAssign<T> for Imp<T> {
        fn shr_assign(&mut self, other: T) {
            self.v.borrow_mut().shr_assign(other);
        }
    }
}

/*
    Allows the use of the - operator
*/
mod sub_impl {
    use crate::Imp;
    use std::ops::{Deref, Sub};

    impl<T: Sub<T> + Copy + Sub<Output = T>> Sub for Imp<T> {
        type Output = Imp<T>;

        fn sub(self, other: Self) -> Self::Output {
            let a = *self.deref();
            let b = *other.deref();
            let r = a.sub(b);
            Imp::new(r)
        }
    }
    impl<T: Sub<T> + Copy + Sub<Output = T>> Sub<T> for Imp<T> {
        type Output = Imp<T>;

        fn sub(self, other: T) -> Self::Output {
            let a = *self.deref();
            let b = other;
            let r = a.sub(b);
            Imp::new(r)
        }
    }
}
/*
    Allows the use of the -= operator
*/
mod sub_assign_impl {
    use crate::Imp;
    use std::ops::SubAssign;

    impl<T: SubAssign<T> + Copy + SubAssign> SubAssign for Imp<T> {
        fn sub_assign(&mut self, other: Imp<T>) {
            self.v.borrow_mut().sub_assign(*other.v.borrow());
        }
    }
    impl<T: SubAssign<T> + Copy + SubAssign> SubAssign<T> for Imp<T> {
        fn sub_assign(&mut self, other: T) {
            self.v.borrow_mut().sub_assign(other);
        }
    }
}

// TODO do we need to implenent Drop?
// Drop            Custom code within the destructor.
