fn main() {
    enums_explaination();
    option_enums();
    match_expression();
    combining_match_with_option_enums();
}

/*
    Let say here enum is storing version of ip address here we can store ip address itself,
    to do that we might take what we learned in struct bcoz we need to create struct underneath enum

    We can also store data with enum variant to store the data inside our variants we'll add parantheses after the variant
    and will specify what type of data we want to store
 */
#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(String)
}

struct IpAddr {
    kind: IpAddressKind,
    address: String
}

// Introducing Message enum: Here we have 4 variants
/*
    We could define these variants as a separate struct like How ever these structs are all of different type The benefit of enum is that all of these variants are grouped
    under the message type just like structs we could define methods and associated functions on our enum type and just like before to do this we'll define an implementation
    on our enum type and just like before to do this we'll define an implementation
*/
#[derive(Debug)]
enum Message {
    Quit, // First variant stores no data
    Move { x: i32, y: i32 }, // second variant stores structs
    Write(String), // Third variant stores single string
    ChangeColor(i32, i32, i32), // Forth variant stores integers
}

impl Message {
    fn some_function() {
        println!("Let's Get Rusty!");
    }
}
fn enums_explaination() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    // We can make this code more concise by keeping data version with address directly in enum only
    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // }



    let message = Message::Move {x:3, y:7};
    let localhost = IpAddressKind::V4(String::from("127.0.0.1"));
    // println!("Local host values: {:#?}, Associated function of enum Message: {:#?}", localhost, message);
}


// fn route(ip_kind: IpAddrKind) {

// }

// =========================================================================================================================
// Option Enums: Many languages have null value and it represents either there will be a value or there will be a no value is that the type system can't guarantee
// That if you use a value its not null, In rust there is no null value instead of that we use option enum
fn option_enums() {
    // It has two variants
    // enum Option<T> {
    //     Some(T), // first is Some which stores value here it's a generic so it could be any value or
    //     None // None which stores no value, So If you have a value that could potentially be null or not exist then you would wrap it in the option enum as we'll
    // } // It is included in program scope by default

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // Like here we have to annotate the type for others rust can infer the type form the values being assigned

    // Example
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // It will throw an error that we cannot add i8 type with Option<i8>
    let sum: i8 = x + y.unwrap_or(0); // To solve above issue we can utilize inbuild function unwrap_or it fetch its value if not found it will set 0 inplace that
}

// =========================================================================================================================================
// Match Expressions It allows us to compare its values against the set of patterns, these patterns could be literals, variables, wild cards
// And many other types which will cover
// Match expression is exhausted meaning that we have to match all possible cases this makes the match expression very useful for enums for example
fn match_expression() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


// ============================================================================================================================================
// Combining Match with option enums
fn combining_match_with_option_enums() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(Some(2));
    println!("Five {:#?}, six {:#?}, none {:#?}", five, six, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x { // Match means we have to match all the possible values
        Some(i) => Some(i + 1),
        _ => None // It will be executed finally like we have to If we will not mention this then we have add condition for none, It will be executed if anything other then Some value comes
    }
}

// =========================================================================================================================================================================
// If-let syntax
fn if_let_usage() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => ()
    }

    // rewriting above code using if-let, Here if some_value matches 3 then print three like above
    if let Some(3) = some_value {
        println!("three");
    }
}

