fn main() {
    // Loop: there are three types of loops in rust
    // This loop will run infinite times until and unless it meets break
    loop {
        println!("Again!!");
        break;
    };
    // we can also return from the loop: semicolon at the end of the loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {}", result);

    // Second type of loop old one
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Third type of loop for in
    let a = [199, 29, 232, 434, 43432, 22, 443, 54];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4) {
        println!("{}!", number);
    }
}