/*
    |        |           
    |        |          {
    |b()    x|-->        "world"  // x is type string which can be dynamic that's why we can't store it in stack we have store it in heap
    |a()  x y|           }
    ---------
    Stack               Heap
    In stack we store
    pointers or binary values
    or addresses

    -> Accessing data on stack is faster as compare to accessing from the heap
*/

fn mainStart() {
    fn a() {
        let x: &str = "hello"; // It is string literal it stores in binary format because it is fixed in size
        let y: i32 = 22; // 
        b();
    }

    fn b() {
        let x: String = String::from("world");
    }
}

fn main() {
    // -------------Ownership rules ----------------
    // 1. Each value in Rust has a variable that's called its owner. Means one variable its one owner
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it's not yet declared
        // let s = "Hello"; // This is string literal
        let s = String::from("Hello"); // s is valid from this point forward
        // do stuff with s
    } // this is now over, and s is no longer valid
}