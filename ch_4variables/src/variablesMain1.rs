fn main() {
    // By default variables are immutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // What is difference in let variable and const variable these are by default immutable and cannot be mutated
    // For constant variable it is recommended to annotate it (u32 unsigned 32 bit)
    // these variable cannot be used for runtime computing as we cannot set as a return value as a constant of a function
    // lastly hundred thousands is hard to read we can write it like 100000 -> 100_000 it works same
    const SUBSCRIBER_COUNT: u32 = 100_000;

    // Shadowing: we can create new variable with same name which is ealier declared with other data types there are two advantages
    // first is we preserve mutability like they are still separatly mutable and immutable
    // secondly we can change its types separatly
    let x = "six";
    println!("The value of x is: {}", x);
}
