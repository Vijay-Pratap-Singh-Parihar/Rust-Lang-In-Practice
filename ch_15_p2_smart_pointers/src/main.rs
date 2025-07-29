// In this part we will be exploring Deref Trait
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn main() {
    let x = 5;
    // let y = &x;
    let y = MyBox::new(x); // While using custom pointer it store in stack not in heap
    // Instead of taking a reference to x we'll create a new box just like our reference box is pointing to a value stored
    // somewhere in memory in this case the value is five so we can use dereferecing operator in same way the only difference here is that y is pointing to a
    // is copy of 5 because if you recall when box is  a smart pointer which implements deref trait
    assert_eq!(5, x);
    // assert_eq!(5, *y);// Here it show error that y property cannot be dereferenced until we implement above using that library
    assert_eq!(5, *(y.deref()));
    // Ques: Why does a deref return a reference instead of the value itself and this has to do with rusts ownership system


    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String
    // We can see no errors thrown though m is of type MyBox and here we're passing in 
    // reference to my box while the hello function expects string slice 
    // we're calling hello and we're passing in a reference to MyBox now what, So MyBox implements deref traits and if we call deref on m
    // we get back a reference to a string the string in a rust standard library also implements the deref trait and if we call deref on a
    // string we get back string slice, rust automatically identifies if different type is passed then the type expected by the function signature
    // At compile time it get the correct type
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
