/*
    Here we will cover grouping related data using structs and covers how to define methods and associated functions on structs and how they compare to tuples
*/

// structs are like object attributes in an object oriented programming langs
#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64
}

fn main() {
    let mut user1 = User {
        email: String::from("vijay@gmail.com"),
        username: String::from("vijaypratap007"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("vijaybhai36");

    let user2 = build_user(
        String::from("vijaypratapsingh@gmail.com"),
        String::from("VijayPratapSinghParihar")
    );

    let user3 = User {
        email: String::from("james@gmail.com"),
        username: String::from("james123"),
        ..user2
    };

    println!("User1 details: {:?}", user1);
    println!("User2 details: {:?}", user2);
    println!("User3 details: {:?}", user3);


    // TUPLE STRUCTS
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);

    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
