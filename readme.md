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