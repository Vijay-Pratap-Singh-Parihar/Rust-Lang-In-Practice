fn main() {
    let s: String = String::from("hello");
    takes_ownership(s); // Here also it moves s to some_string parameter of function of takes_ownership
    // println!("{}", s); // Here it will throw error that s loosed his ownership to another variable use clone wherever it is assigned

    let x = 45;
    makes_copy(x); // Here it will shallow copied as it is stored in stack only, as integers, boolean, character, string literal are shallow copied
    println!("x => {}", x); // here it will do nothing

    let s1 = gives_ownership(); // This Function will return new variable which will be type of string
    println!("s1 = {}", s1);

    let s2 = String::from("Takes back");
    let s3 = takes_and_gives_back(s2); // Here we are just moving whole s2 content back to s3 variable if we want to copy then we have to use clone here
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);// After this scope is utilized that s moved to some_string then some_string is dropped
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("Hello");

    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}