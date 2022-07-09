![crates.io](https://img.shields.io/crates/v/interior_mutability_pointer.svg)
![downloads](https://img.shields.io/crates/d/interior_mutability_pointer)
![docs.rs](https://img.shields.io/docsrs/interior_mutability_pointer)

# Safety
The `DerefMut` implementation is unsound due to this library essentially working around the runtime safety provided
by using `RefCell`. See [Issue #2](https://github.com/samhamnam/interior_mutability_pointer/issues/2).\
Due to this `Imp::new(..)` has been marked as `unsafe`.

# Interior Mutability Pointer
A wrapper around `Rc<RefCell<T>>` allowing immediate access to inner methods,
without the need for `.borrow()` or `.borrow_mut()`,
allowing for a more seamless pointer experience.
```rs
let mut k = Imp::new(String::new());
let p = k.clone(); // Clone the pointer.
k.push_str("yo");
println!("{} {}", k, p); // Prints "yo yo"
```
Also allows the use of operators:
```rs
let mut k = Imp::new(5);
let p = k.clone(); // Clone the pointer.
k += 5;
println!("{} {}", k, p); // Prints "10 10"
```
The biggest difference to `Rc<RefCell<T>>` is that your pointer instance will need to be marked as `mut`
if you want to use `&mut self` methods, as opposed to `Rc<RefCell<T>>` instances where you can call `.borrow_mut()`,
removing the need for the mut keyword.
However, this does not mean that all clones of the pointer need to be mutable!
```rs
let k = Imp::new(String::new());
let mut p = k.clone(); // Clone the pointer.
p.push_str("yo");
println!("{:?} {:?}", k, p); // Prints "yo yo"
```
Also supports dynamic dispatch for all your trait ojects, in both mutable and inmutable contexts!
```rs
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
```
```rs
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

println!("{:?}", rc_refcell); // Prints [20, 30]
println!("{:?}", imp);        // Prints [20, 30]
```