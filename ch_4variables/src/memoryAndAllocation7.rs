fn main() {
    let x = "2222"; // i32 type variable these types like character, bool, integer, including string literals
    let y = x; // will be copied if assigned to new variable, whatever stored in stack in form of binary
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // if s2 = s1 then s1 is moved to s2 its not copied 
    // Like above it cannot be copied as it is stored in heap which means s1 pointer is pointing to string now s2 will start pointing to its string
    // For that rust provides clone feature which will be used to copy or said to clone .clone()
    println!("The value of s1: {}", s1);
    println!("The value of s2: {}", s2);
}