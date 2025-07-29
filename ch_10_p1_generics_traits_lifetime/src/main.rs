use std::cmp::PartialOrd;
fn main() {
    let vec_list = vec![21, 34, 32, 552, 442, 12];

    let largest = get_largest(vec_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['a', 'c', 'f', 'Z'];

    let largest = get_largest(char_list);

    println!("The largest char is {}", largest);


    // We can also use generics with struct
    generics_with_structs();
}

// There is no need for finding two types of function for the same use like largest number or largest character
// for that rust introduces generics here from this fn get_largest_num(number_list: Vec<i32>) -> i32 { to below line 
// its declared like this <T> after function names, we use generic where only arguement datatypes are different and whole logic will be same
// I have added PartialOrd + Copy because PartialOrd will restrict to T type that can be ordered and COPY a type that we could copy
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// fn get_largest_char(characters_list: Vec<char>) -> char {
//     let mut largest = characters_list[0];
//     for charec in characters_list {
//         if charec > largest {
//             largest = charec
//         }
//     }
//     largest
// }

// ===============================================================
// As functions we specify generics after the name of struct here we are using two different generics types 
struct Point<T, U> {
    x: T,
    y: U
}


fn generics_with_structs() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5, y: 10.0 };

    // We can also use generics in enum, Here we have two example Option enum and Result enum
    enum Option<T> {
        Some(T),
        None
    }

    enum Result<T, E> {
        Ok(T),
        Err(E)
    }

    generics_with_struct_methods();
}

struct Point1 <T> {
    x: T,
    y: T
}

// When we Initialize implementation methods for struct we have to mention generics type like below everytime generic type can differ but it will refer to same multi type datatype
impl<T: Copy> Point1<T> {
    fn x(&self) -> T {
        self.y
    }
}

// above one is dynamic/generics type method and below one is static float 64 implementation method
impl Point1<f64> {
    fn y(&self) -> f64 {
        self.y
    }
} 

fn generics_with_struct_methods() {
    let p = Point1 { x: 5, y: 10 };
    p.x();

    let p1 = Point1 { x: 5.0, y: 1.0 };
    p1.y();


    more_complex_example();
}

//=============================================================================
// More deep example of above
struct Point2<T, U> {
    x: T,
    y: U
}

impl<T: PartialOrd + Copy, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}


fn more_complex_example() {
    let p1 = Point2 { x: 5, y: 10.5 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    performance_impact();
}

// ===========================================================================
// Performance generics Impact
// Instead of creating two Option enum for two differenct variables we can create single Option enum with generics
// Using that It will not hit to performance as it will create separate Option Enum at a compile time

enum Option<T> {
    Some(T),
    None
}

fn performance_impact() {
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}
