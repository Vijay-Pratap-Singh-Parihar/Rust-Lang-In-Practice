// fn main() {
//     let b = Box::new(5);
//     println!("b = {}", b);
// }

// Cons List is a datastructure that comes with programming language
// This is not commonly used datastructure we use here for boxes can allow recursive data
#[derive(Debug)]
enum List {
    // Cons(i32, List), // This will give error as it is dynamic to occupy storage which is recursive indirection,
    Cons(i32, Box<List>), // This will solve our issue as We have wrapped with Box smartpointer and it will now have fix space which will directed to list in heap 
    Nil,
}

// How to bring list variance into the scope
use List::{Cons, Nil};

fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); // Now instead of like above we store box smart pointer
    println!("List {:?}", list);
}