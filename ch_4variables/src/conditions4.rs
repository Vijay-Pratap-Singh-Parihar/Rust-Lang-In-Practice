fn main() {
    // Control flow
    let number = 5;

    // if number {  // This will through an error, because number should be an boolean
    if number < 10 {
        println!("First condition was true");
    }
    else if number < 22 {
        println!("second condition was true");
    }
    else {
        println!("Condition was false");
    }

    // 2nd example
    let condition = true;
    let number = if condition { 5 } else { 6 };
}