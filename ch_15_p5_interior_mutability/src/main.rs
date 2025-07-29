// Ref cell smart pointer It represents single ownership over the data it holds much like a smart pointers the difference is the
// box pointer enforces borrowing rules at compile time where ref cell enforces borrowing rules at run time
// this means if you break the borrowing rules at runtime your program will panic and exit the advantage of checking the borrowing rules
// at the compile time errors will be caught sooner in the development cycle
// most famous example is halting problem, You can only use ref cell on a single threaded program

/*
- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners
- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time;
    RefCell<T> allows immutable or mutable borrows checked at runtime
- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the calue inside the RefCell<T> even when the RefCell<T> is immutable
*/

// fn main() {
//     println!("Hello, world!");
// }
/*
## RefCell<T> ko 'Elite' Level pe Samajhna: The Runtime Bouncer 👮
Sochne ka sahi tareeka yeh hai: Rust ka borrow checker ek bahut hi strict gatekeeper hai jo program compile hone se pehle hi saare rules check kar leta hai. Lekin kabhi-kabhi hum aisa code likhte hain jo humein pata hai ki safe hai, par compiler uski safety prove nahi kar paata aur error de deta hai.

The core idea: RefCell<T> is a way to tell the compiler, "Don't worry about the borrowing rules for this value right now. I'll take the responsibility, and we'll check the rules at runtime."

RefCell<T> borrow checker ko compile-time se utha kar run-time par le jaata hai.

### 1. Problem Kya Hai? Compiler Thoda Over-Protective Hai
The MockMessenger example is perfect. The Messenger trait ka contract hai ki send method &self lega (yaani immutable). Lekin hamare test ke liye, humein send ke andar sent_messages vector ko change karna hai.

Rust

trait Messenger {
    fn send(&self, msg: &str); // Contract says immutable!
}

struct MockMessenger {
    sent_messages: Vec<String>,
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // ERROR! `self` is immutable, but `push` needs a mutable borrow.
        self.sent_messages.push(String::from(message));
    }
}
Yahan compile-time gatekeeper (borrow checker) humein aage nahi badhne dega.

### 2. The RefCell<T> Solution: "Compiler, Trust Me Bro"
RefCell<T> is unsafe code ko ek safe wrapper mein daal kar yeh problem solve karta hai. Yeh "Interior Mutability" pattern ko enable karta hai—yaani ek aisi cheez ke andar mutate karna jo bahar se immutable dikhti hai.

Hum sent_messages ko RefCell mein wrap kar dete hain:

Rust

use std::cell::RefCell;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>, // Wrapped in RefCell
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // No error! `self` is still immutable.
        // We ask the RefCell for a mutable borrow at RUNTIME.
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}
Ab compiler ko koi problem nahi hai. RefCell ke paas do main methods hain:

.borrow(): Runtime par ek immutable borrow (&T) maangta hai.

.borrow_mut(): Runtime par ek mutable borrow (&mut T) maangta hai.

RefCell andar hi andar ek counter rakhta hai. Agar aapne rules tode (e.g., do borrow_mut() ek saath maang liye), toh compiler error nahi dega, lekin aapka program run-time par panic! ho jaayega.

### 3. The Price of Flexibility (The Trade-Off)
RefCell<T> koi free magic nahi hai. Iske trade-offs hain:

Pro: Aapko aisi flexibility milti hai jo compile-time checker nahi de sakta.

Con: Borrowing errors compile-time par pakde jaane ki jagah, run-time par panics ke roop mein saamne aate hain. Iska matlab hai ki bug aapke production code tak pahunch sakta hai.

Con: Runtime par borrow count track karne ka ek chota sa performance cost hota hai.

### 4. The Power Combo: Rc<RefCell<T>> 🦸‍♂️+🦸
Ab sabko ek saath jodte hain. Hamare paas do alag-alag heroes the:

Rc<T>: Multiple owners, par sirf read-only. (Library ki book jise sab padh sakte hain).

RefCell<T>: Single owner, par interior mutability. (Ek akele aadmi ki magical diary jisme wo likh sakta hai).

What if you need multiple owners who can also mutate the data?
Aap dono ki powers ko combine karte hain: Rc<RefCell<T>>

Rc<RefCell<T>> is a value that is shared among multiple owners, and any of those owners can mutate it.

Analogy: Rc<RefCell<T>> ek aisi library book (Rc) hai jo asal mein ek magical notebook (RefCell) hai. Kai log isse issue (clone) kar sakte hain, aur koi bhi usmein likh (borrow_mut) sakta hai. Lekin notebook yeh dhyaan rakhegi ki ek time par sirf ek hi insaan likh raha ho. Agar do log ek saath likhne ki koshish karenge, notebook panic kar jaayegi.

### The "Elite" Takeaway
RefCell<T> ownership ke rules ko todta nahi hai, balki unhe run-time par check karta hai.

Yeh Interior Mutability pattern ka aadhar hai, jisse aap immutable values ke andar data modify kar sakte hain.

Yeh compile-time safety aur run-time flexibility ke beech ek trade-off hai.

Rc<RefCell<T>> ek behad common aur powerful Rust pattern hai jo aapko shared mutable state provide karta hai, jo ki complex applications ke liye zaroori hai. */



#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}