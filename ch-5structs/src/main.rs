#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// impl -> implementation block helps us with functions and methods
// Here methods are similar to functions but structs tied to methods 
impl Rectangle { // implementation block
    // method defination: first argument is always self which is the instance the method is being called on "here we are taking reference of Rectangle Instance"
    // We could also take immutable reference or in a rare cases we could take ownership of the instance, In this case we just need a reference 
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

// Also we can write its associated function inside a implementation block where we dont have to pass self as a argument
// Here we can create multiple implementation block for same structs rust allow us to create multiple implementation block
impl Rectangle {
    fn square(size: u32) -> Rectangle { // This is our associated function which cannot be excessed from outside
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    // let width = 30;
    // let height = 50;
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let sec_rect = Rectangle {
        width: 20,
        height: 40
    };

    let third_rect = Rectangle::square(25);

    println!(
        "The area of the rectangle is {:#?} square pixels. If rectangle one can hold rectangle two: {}, third rect with associated function {:#?}",
        rect.area(), // Here we use dot notation to call the method as rust has a feature called automatic referencing and dereferencing where in other languages there is not such like feature
        rect.can_hold(&sec_rect),
        third_rect
    );
}

// fn area(width: u32, height: u32) -> u32 {
//     height * width
// }

// here we are using tuples struct, here is new 
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Here we are utilizing rectangle struct
// fn area(dimensions: &Rectangle) -> u32 {
//     dimensions.height * dimensions.width
// }