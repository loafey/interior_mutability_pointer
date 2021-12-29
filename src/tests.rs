#![allow(unused_imports)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::float_cmp)]

#[cfg(feature = "compile_failure")]
mod compile_failures {
    use std::ops::DerefMut;

    use crate::Imp;

    #[test]
    fn mem_drop() {
        let _ = {
            let i = Imp::new(vec![0]);
            *i
        };
    }

    #[test]
    fn mut_mem_drop() {
        let _ = {
            let mut i = Imp::new(vec![0]);
            i.deref_mut()
        };
    }
}

// Test equality
mod ptr_eq {
    use crate::Imp;

    #[test]
    fn non_equal() {
        let p1 = Imp::new(String::new());
        let p2 = Imp::new(String::new());
        assert!(!Imp::ptr_eq(&p1, &p2));
    }

    #[test]
    fn equal() {
        let p1 = Imp::new(String::new());
        let p2 = p1.clone();
        assert!(Imp::ptr_eq(&p1, &p2));
    }
}

// Just ensure this compiles, as it is possible with Rc<RefCell<T>>
// and should work with Imp<T>
mod dynamic_dispatch {
    use std::{cell::RefCell, ops::Deref, rc::Rc};

    use crate::Imp;

    trait Animal {
        fn sound(&self) -> &'static str;
        fn volume(&self) -> i32;
        fn set_volume(&mut self, v: i32);
    }

    #[derive(Clone, Copy)]
    struct Sheep {
        volume: i32,
    }
    impl Animal for Sheep {
        fn sound(&self) -> &'static str {
            "baah"
        }

        fn volume(&self) -> i32 {
            self.volume
        }

        fn set_volume(&mut self, v: i32) {
            self.volume = v;
        }
    }

    #[derive(Clone, Copy)]
    struct Dog {
        volume: i32,
    }
    impl Animal for Dog {
        fn sound(&self) -> &'static str {
            "bark"
        }

        fn volume(&self) -> i32 {
            self.volume
        }

        fn set_volume(&mut self, v: i32) {
            self.volume = v;
        }
    }

    #[test]
    fn test() {
        let s = Sheep { volume: 10 };
        let d = Dog { volume: 15 };
        let rc_refcell: Vec<Rc<RefCell<dyn Animal>>> =
            vec![Rc::new(RefCell::new(s)), Rc::new(RefCell::new(d))];
        let rc: Vec<Rc<dyn Animal>> = vec![Rc::new(s), Rc::new(d)];
        let imp: Vec<Imp<dyn Animal>> = vec![Imp::new(s), Imp::new(d)];

        let rc_refcell = rc_refcell
            .iter()
            .map(|p| p.borrow().sound())
            .collect::<Vec<_>>();
        let rc = rc.iter().map(|p| p.sound()).collect::<Vec<_>>();
        let imp = imp.iter().map(|p| p.sound()).collect::<Vec<_>>();

        assert!((rc_refcell == rc) && (rc == imp));
    }

    #[test]
    fn test_mut() {
        let s = Sheep { volume: 10 };
        let d = Dog { volume: 15 };

        let mut rc_refcell: Vec<Rc<RefCell<dyn Animal>>> =
            vec![Rc::new(RefCell::new(s)), Rc::new(RefCell::new(d))];
        let mut imp: Vec<Imp<dyn Animal>> = vec![Imp::new(s), Imp::new(d)];

        rc_refcell.iter_mut().for_each(|a| {
            let v = a.borrow().volume();
            a.borrow_mut().set_volume(v * 2);
        });

        imp.iter_mut().for_each(|a| {
            let v = a.volume();
            a.set_volume(v * 2);
        });

        let rc_refcell = rc_refcell
            .iter()
            .map(|p| p.borrow().volume())
            .collect::<Vec<_>>();
        let imp = imp.iter().map(|p| p.volume()).collect::<Vec<_>>();

        assert_eq!(&rc_refcell, &[20, 30]);
        assert_eq!(&imp, &[20, 30]);
        assert!(rc_refcell == imp);
    }
}

mod clone_without_t {
    use crate::Imp;
    #[derive(Debug)]
    struct NonCloneable {
        _val: i32,
    }

    #[test]
    fn clone_test() {
        let nc = NonCloneable { _val: 0 };
        let mut i = Imp::new(nc);
        let p = i.clone();

        i._val += 100;

        assert_eq!(i._val, p._val);
    }
}

mod display {
    use crate::Imp;

    #[test]
    fn debug_string_ref() {
        let data = vec![1, 2, 3];
        let ptr = Imp::new(data.clone());

        let data_str = format!("{:?}", data);
        let ptr_str = format!("{:?}", ptr);

        assert_eq!(data_str, ptr_str);
    }

    #[test]
    fn display_string_ref() {
        let data = "Hello :)".to_owned();
        let ptr = Imp::new(data.clone());

        let data_str = data;
        let ptr_str = format!("{}", ptr);

        assert_eq!(data_str, ptr_str);
    }
}

mod eq {
    use std::str::FromStr;

    use crate::Imp;

    #[test]
    fn eq_inner_ref() {
        let data = 6;
        let ptr = Imp::new(data);
        assert_eq!(ptr, data);
    }

    #[test]
    fn ne_inner_ref() {
        let data = 6;
        let ptr = Imp::new(5);
        assert_ne!(ptr, data);
    }

    #[test]
    fn ne_ref() {
        let p1 = Imp::new(6);
        let p2 = Imp::new(5);
        assert_ne!(p1, p2);
    }

    #[test]
    fn eq_ref() {
        let p1 = Imp::new(5);
        let p2 = Imp::new(5);

        assert_eq!(p2, p1);
    }

    #[test]
    fn eq_modify_ref() {
        let mut p1 = Imp::new(String::from_str("2").unwrap());
        let p2 = p1.clone();
        p1.push('1');
        assert_eq!(p1, p2);
    }
}
mod order_box {
    use crate::Imp;

    #[test]
    fn less_ref() {
        let p1 = Imp::new(4);
        let p2 = Imp::new(5);

        assert!(p1 < p2);
    }
    #[test]
    fn less2_ref() {
        let p1 = Imp::new(4);
        let p2 = Imp::new(5);

        assert!(p2 >= p1);
    }
    #[test]
    fn greater_ref() {
        let p1 = Imp::new(6);
        let p2 = Imp::new(5);

        assert!(p1 > p2);
    }
    #[test]
    fn greater2_ref() {
        let p1 = Imp::new(6);
        let p2 = Imp::new(5);

        assert!(p2 <= p1);
    }

    #[test]
    fn lesser_or_eq_ref() {
        let p1 = Imp::new(5);
        let p2 = Imp::new(5);

        assert!(p1 <= p2);
    }
    #[test]
    fn greater_or_eq_ref() {
        let p1 = Imp::new(6);
        let p2 = Imp::new(5);

        assert!(p2 < p1);
    }
}

mod order_inner {
    use crate::Imp;

    #[test]
    fn less_ref() {
        let p1 = Imp::new(4);
        let p2 = 5;

        assert!(p1 < p2);
    }
    #[test]
    fn less2_ref() {
        let p1 = Imp::new(4);
        let p2 = 3;

        assert!(p1 >= p2);
    }
    #[test]
    fn greater_ref() {
        let p1 = Imp::new(6);
        let p2 = 5;

        assert!(p1 > p2);
    }
    #[test]
    fn greater2_ref() {
        let p1 = Imp::new(6);
        let p2 = 7;

        assert!(p1 <= p2);
    }

    #[test]
    fn lesser_or_eq_ref() {
        let p1 = Imp::new(5);
        let p2 = 5;

        assert!(p1 <= p2);
    }
    #[test]
    fn greater_or_eq_ref() {
        let p1 = Imp::new(6);
        let p2 = 7;

        assert!(p1 < p2);
    }
}

mod index {
    use crate::Imp;

    #[test]
    fn indexable() {
        let v = Imp::new(vec![1, 2, 3, 4]);
        assert_eq!(v[1], 2);
    }

    #[test]
    fn index_mut() {
        let mut v = Imp::new(vec![1, 2, 3, 4]);
        v[1] = 5;
        assert_eq!(v[1], 5);
    }
}

mod range {
    use crate::Imp;

    #[test]
    fn range() {
        let r = Imp::new("yood");
        let t = &r[1..3];
        assert_eq!(t, "oo")
    }

    #[test]
    fn range_mut() {
        let mut r = Imp::new(vec![String::new(); 5]);
        r[0..2].iter_mut().for_each(|s| s.push('d'));
        assert_eq!(
            r,
            vec![
                "d".to_owned(),
                "d".to_owned(),
                String::new(),
                String::new(),
                String::new()
            ]
        );
    }
}

mod add {
    use crate::Imp;

    #[test]
    fn add() {
        let mut p = Imp::new(1);
        p = p + 1;
        assert_eq!(p, 2);
    }

    #[test]
    fn add_ref() {
        let mut p1 = Imp::new(1);
        let p2 = Imp::new(1);
        p1 = p1 + p2;
        assert_eq!(p1, 2);
    }

    #[test]
    fn add_assign() {
        let mut p = Imp::new(1);
        p += 1;
        assert_eq!(p, 2);
    }

    #[test]
    fn add_assign_ref() {
        let mut p1 = Imp::new(1);
        let p2 = Imp::new(1);
        p1 += p2;
        assert_eq!(p1, 2);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(2);
        let p2 = p1.clone();
        p1 += 2;
        assert_eq!(p1, p2)
    }
}

mod bitand {
    use crate::Imp;

    #[test]
    fn bitand_true() {
        let p1 = Imp::new(true);
        let p2 = true;
        assert_eq!(p1 & p2, true);
    }
    #[test]
    fn bitand_false() {
        let p1 = Imp::new(true);
        let p2 = false;
        assert_ne!(p1 & p2, true);
    }

    #[test]
    fn bitand_true_ref() {
        let p1 = Imp::new(true);
        let p2 = Imp::new(true);
        assert_eq!(p1 & p2, true);
    }

    #[test]
    fn bitand_false_ref() {
        let p1 = Imp::new(true);
        let p2 = Imp::new(false);
        assert_eq!(p1 & p2, false);
    }
}

mod bitand_assign {
    use crate::Imp;

    #[test]
    fn bitand_true() {
        let mut p1 = Imp::new(true);
        p1 &= true;
        assert_eq!(p1, true);
    }
    #[test]
    fn bitand_false() {
        let mut p1 = Imp::new(true);
        p1 &= false;
        assert_ne!(p1, true);
    }

    #[test]
    fn bitand_true_ref() {
        let mut p1 = Imp::new(true);
        p1 &= Imp::new(true);
        assert_eq!(p1, true);
    }

    #[test]
    fn bitand_false_ref() {
        let mut p1 = Imp::new(true);
        p1 &= Imp::new(false);
        assert_eq!(p1, false);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(true);
        let p2 = p1.clone();
        p1 &= false;
        assert_eq!(p1, p2)
    }
}

mod bitor {
    use crate::Imp;

    #[test]
    fn bitor_true() {
        let p1 = Imp::new(true);
        let p2 = true;
        assert_eq!(p1 | p2, true);
    }
    #[test]
    fn bitor_true_false() {
        let p1 = Imp::new(true);
        let p2 = false;
        assert_eq!(p1 | p2, true);
    }

    #[test]
    fn bitor_false_false() {
        let p1 = Imp::new(false);
        let p2 = false;
        assert_eq!(p1 | p2, false);
    }

    #[test]
    fn bitor_true_true_ref() {
        let p1 = Imp::new(true);
        let p2 = Imp::new(true);
        assert_eq!(p1 | p2, true);
    }

    #[test]
    fn bitor_true_false_ref() {
        let p1 = Imp::new(true);
        let p2 = Imp::new(false);
        assert_eq!(p1 | p2, true);
    }

    #[test]
    fn bitor_false_false_ref() {
        let p1 = Imp::new(false);
        let p2 = Imp::new(false);
        assert_eq!(p1 | p2, false);
    }
}

mod bitor_assign {
    use crate::Imp;

    #[test]
    fn bitor_true() {
        let mut p1 = Imp::new(true);
        p1 |= true;
        assert_eq!(p1, true);
    }
    #[test]
    fn bitor_false() {
        let mut p1 = Imp::new(true);
        p1 |= false;
        assert_eq!(p1, true);
    }

    #[test]
    fn bitor_true_ref() {
        let mut p1 = Imp::new(false);
        p1 |= Imp::new(false);
        assert_eq!(p1, false);
    }

    #[test]
    fn bitor_false_ref() {
        let mut p1 = Imp::new(false);
        p1 |= Imp::new(true);
        assert_eq!(p1, true);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(false);
        let p2 = p1.clone();
        p1 |= true;
        assert_eq!(p1, p2)
    }
}

mod bitxor {
    use crate::Imp;

    #[test]
    fn bitxor_true() {
        let p1 = Imp::new(true);
        let p2 = true;
        assert_eq!(p1 ^ p2, false);
    }
    #[test]
    fn bitxor_true_false() {
        let p1 = Imp::new(true);
        let p2 = false;
        assert_eq!(p1 ^ p2, true);
    }

    #[test]
    fn bitxor_false_false() {
        let p1 = Imp::new(false);
        let p2 = false;
        assert_eq!(p1 ^ p2, false);
    }

    #[test]
    fn bitxor_true_true_ref() {
        let p1 = Imp::new(true);
        let p2 = Imp::new(true);
        assert_eq!(p1 ^ p2, false);
    }

    #[test]
    fn bitxor_true_false_ref() {
        let p1 = Imp::new(true);
        let p2 = Imp::new(false);
        assert_eq!(p1 ^ p2, true);
    }

    #[test]
    fn bitxor_false_false_ref() {
        let p1 = Imp::new(false);
        let p2 = Imp::new(false);
        assert_eq!(p1 ^ p2, false);
    }
}

mod bitxor_assign {
    use crate::Imp;

    #[test]
    fn bitxor_true() {
        let mut p1 = Imp::new(true);
        p1 ^= true;
        assert_eq!(p1, false);
    }
    #[test]
    fn bitxor_false() {
        let mut p1 = Imp::new(true);
        p1 ^= false;
        assert_eq!(p1, true);
    }

    #[test]
    fn bitxor_true_ref() {
        let mut p1 = Imp::new(false);
        p1 ^= Imp::new(false);
        assert_eq!(p1, false);
    }

    #[test]
    fn bitxor_false_ref() {
        let mut p1 = Imp::new(false);
        p1 ^= Imp::new(true);
        assert_eq!(p1, true);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(false);
        let p2 = p1.clone();
        p1 ^= true;
        assert_eq!(p1, p2)
    }
}

mod div {
    use crate::Imp;

    #[test]
    fn div() {
        let p1 = Imp::new(10);
        let p2 = 2;
        assert_eq!(p1 / p2, 5);
    }

    #[test]
    fn div_float() {
        let p1 = Imp::new(10.0);
        let p2 = 2.0;
        assert_eq!(p1 / p2, 5.0);
    }

    #[test]
    fn div_ref() {
        let p1 = Imp::new(10);
        let p2 = Imp::new(5);
        assert_eq!(p1 / p2, 2);
    }

    #[test]
    fn div_float_ref() {
        let p1 = Imp::new(10.0);
        let p2 = Imp::new(5.0);
        assert_eq!(p1 / p2, 2.0);
    }
}

mod div_assign {
    use crate::Imp;

    #[test]
    fn div() {
        let mut p1 = Imp::new(10);
        p1 /= 2;
        assert_eq!(p1, 5);
    }
    #[test]
    fn div_float() {
        let mut p1 = Imp::new(10.0);
        p1 /= 2.0;
        assert_eq!(p1, 5.0);
    }

    #[test]
    fn div_ref() {
        let mut p1 = Imp::new(10);
        p1 /= Imp::new(2);
        assert_eq!(p1, 5);
    }

    #[test]
    fn div_float_ref() {
        let mut p1 = Imp::new(10.0);
        p1 /= Imp::new(2.0);
        assert_eq!(p1, 5.0);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(10);
        let p2 = p1.clone();
        p1 /= 2;
        assert_eq!(p1, p2)
    }
}

mod not {
    use crate::Imp;

    #[test]
    fn not_true() {
        let p1 = Imp::new(true);
        let p2 = !p1.clone();
        assert_ne!(p1, p2);
    }
    #[test]
    fn not_false() {
        let p1 = Imp::new(true);
        let p2 = !p1.clone();
        assert_ne!(p1, p2);
    }
}

mod fn_test {
    // Just here to show that you can put a closure in an Imp.
    use crate::Imp;
    #[test]
    fn fn_test() {
        let mut _k = 5;
        let mut p = Imp::new(move || {
            _k += 1;
            println!("yo")
        });
        p();
    }
}

mod mul {
    use crate::Imp;

    #[test]
    fn mul() {
        let p1 = Imp::new(2);
        let p2 = 5;
        assert_eq!(p1 * p2, 10);
    }
    #[test]
    fn mul_float() {
        let p1 = Imp::new(2.0);
        let p2 = 5.0;
        assert_eq!(p1 * p2, 10.0);
    }

    #[test]
    fn mul_ref() {
        let p1 = Imp::new(2);
        let p2 = Imp::new(5);
        assert_eq!(p1 * p2, 10);
    }

    #[test]
    fn mul_float_ref() {
        let p1 = Imp::new(2.0);
        let p2 = Imp::new(5.0);
        assert_eq!(p1 * p2, 10.0);
    }
}

mod mul_assign {
    use crate::Imp;

    #[test]
    fn mul() {
        let mut p1 = Imp::new(2);
        p1 *= 5;
        assert_eq!(p1, 10);
    }
    #[test]
    fn mul_float() {
        let mut p1 = Imp::new(2.0);
        p1 *= 5.0;
        assert_eq!(p1, 10.0);
    }

    #[test]
    fn mul_ref() {
        let mut p1 = Imp::new(2);
        p1 *= Imp::new(5);
        assert_eq!(p1, 10);
    }

    #[test]
    fn mul_float_ref() {
        let mut p1 = Imp::new(2.0);
        p1 *= Imp::new(5.0);
        assert_eq!(p1, 10.0);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(2.0);
        let p2 = p1.clone();
        p1 *= 5.0;
        assert_eq!(p1, p2)
    }
}

mod neg {
    use crate::Imp;

    #[test]
    fn neg_true() {
        let p1 = Imp::new(10);
        let p2 = -p1.clone();
        assert_ne!(p1, p2);
    }
    #[test]
    fn neg_false() {
        let p1 = Imp::new(10);
        let p2 = -p1.clone();
        assert_ne!(p1, p2);
    }
}

mod rangebounds {
    use std::ops::Bound::{Excluded, Included, Unbounded};
    use std::ops::RangeBounds;

    use crate::Imp;

    #[test]
    fn startbound() {
        let p = Imp::new(0..10);
        assert_eq!(p.start_bound(), Included(&0))
    }
    #[test]
    fn endbound() {
        let p = Imp::new(0..10);
        assert_eq!(p.end_bound(), Excluded(&10))
    }

    #[test]
    fn unbound_startbound() {
        let p = Imp::new(..10);
        assert_eq!(p.start_bound(), Unbounded)
    }
    #[test]
    fn unbound_endbound() {
        let p = Imp::new(0..);
        assert_eq!(p.end_bound(), Unbounded)
    }

    #[test]
    fn contains() {
        let p = Imp::new(0..5);
        assert!(p.contains(&2));
    }

    #[test]
    fn not_contains() {
        let p = Imp::new(0..5);
        assert!(!p.contains(&6));
    }
}

mod rem {
    use crate::Imp;

    #[test]
    fn rem() {
        let p1 = Imp::new(10);
        let p2 = 8;
        assert_eq!(p1 % p2, 2);
    }
    #[test]
    fn rem_float() {
        let p1 = Imp::new(10.0);
        let p2 = 8.0;
        assert_eq!(p1 % p2, 2.0);
    }

    #[test]
    fn rem_ref() {
        let p1 = Imp::new(10);
        let p2 = Imp::new(8);
        assert_eq!(p1 % p2, 2);
    }

    #[test]
    fn rem_float_ref() {
        let p1 = Imp::new(10.0);
        let p2 = Imp::new(8.0);
        assert_eq!(p1 % p2, 2.0);
    }
}

mod rem_assign {
    use crate::Imp;

    #[test]
    fn rem() {
        let mut p1 = Imp::new(10);
        p1 %= 8;
        assert_eq!(p1, 2);
    }
    #[test]
    fn rem_float() {
        let mut p1 = Imp::new(10.0);
        p1 %= 8.0;
        assert_eq!(p1, 2.0);
    }

    #[test]
    fn rem_ref() {
        let mut p1 = Imp::new(10);
        p1 %= Imp::new(8);
        assert_eq!(p1, 2);
    }

    #[test]
    fn rem_float_ref() {
        let mut p1 = Imp::new(10.0);
        p1 %= Imp::new(8.0);
        assert_eq!(p1, 2.0);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(10.0);
        let p2 = p1.clone();
        p1 %= 8.0;
        assert_eq!(p1, p2)
    }
}

mod shl {
    use crate::Imp;

    #[test]
    fn shl() {
        let p1 = Imp::new(8);
        let p2 = 1;
        assert_eq!(p1 << p2, 16);
    }

    #[test]
    fn shl_ref() {
        let p1 = Imp::new(8);
        let p2 = Imp::new(1);
        assert_eq!(p1 << p2, 16);
    }
}

mod shl_assign {
    use crate::Imp;

    #[test]
    fn shl() {
        let mut p1 = Imp::new(8);
        p1 <<= 1;
        assert_eq!(p1, 16);
    }

    #[test]
    fn shl_ref() {
        let mut p1 = Imp::new(8);
        p1 <<= Imp::new(1);
        assert_eq!(p1, 16);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(8);
        let p2 = p1.clone();
        p1 <<= 2;
        assert_eq!(p1, p2)
    }
}

mod shr {
    use crate::Imp;

    #[test]
    fn shl() {
        let p1 = Imp::new(8);
        let p2 = 1;
        assert_eq!(p1 >> p2, 4);
    }

    #[test]
    fn shl_ref() {
        let p1 = Imp::new(8);
        let p2 = Imp::new(1);
        assert_eq!(p1 >> p2, 4);
    }
}

mod shr_assign {
    use crate::Imp;

    #[test]
    fn shl() {
        let mut p1 = Imp::new(8);
        p1 >>= 1;
        assert_eq!(p1, 4);
    }

    #[test]
    fn shl_ref() {
        let mut p1 = Imp::new(8);
        p1 >>= Imp::new(1);
        assert_eq!(p1, 4);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(8);
        let p2 = p1.clone();
        p1 >>= 2;
        assert_eq!(p1, p2)
    }
}

mod sub {
    use crate::Imp;

    #[test]
    fn sub() {
        let mut p = Imp::new(1);
        p = p - 1;
        assert_eq!(p, 0);
    }

    #[test]
    fn sub_ref() {
        let mut p1 = Imp::new(1);
        let p2 = Imp::new(1);
        p1 = p1 - p2;
        assert_eq!(p1, 0);
    }

    #[test]
    fn sub_assign() {
        let mut p = Imp::new(1);
        p -= 1;
        assert_eq!(p, 0);
    }

    #[test]
    fn sub_assign_ref() {
        let mut p1 = Imp::new(1);
        let p2 = Imp::new(1);
        p1 -= p2;
        assert_eq!(p1, 0);
    }

    #[test]
    fn clone_assign() {
        let mut p1 = Imp::new(2);
        let p2 = p1.clone();
        p1 -= 2;
        assert_eq!(p1, p2)
    }
}
