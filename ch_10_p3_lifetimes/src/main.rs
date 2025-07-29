fn main() {
    // let r;
    // {
    //      let x = 5;
    //      r = &x; // It will through error as x life will be completed here
    // }
    // println!("r: {}", r);// Here r is having dangling reference
    
    let x = 5;

    let r = &x;

    println!("r: {}", r);

    // Generical Lifetime Annotations
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    // longest function will return the string slice 
    // So all the borrower checker here will have a 
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    lifetime_annotation_with_structs();
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Here <'a> -> 'a is generical lifetime annotation where a will be a name of annotation which will be utilized
// And x is using 'a lifetime and y is also using 'a lifetime
// generic lifetime annotation don't actually change lifetime they just create relationships between the lifetimes of multiple references so here what we're actually saying is
// there is relationship between x and y and the return value that relationship is this the lifetime of the returned references will be the same as smallest lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}


// Lifetime annotation with structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_annotation_with_structs() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

//===========================================================================================================
// lifetime ellision
// RULES

// 1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime parameter, that lifetime is
//      assigned to all output lifetime parameters;

// 3. If there are multiple input lifetime parameters, but one of them
//       is &self or &mut self the lifetime of self is assigned to all output


// As per rule 1 s will get first parameter, 2nd rule so in function return type 
// So the 3rd rule is only applied to methods and it will make a little bit more sense, You can see below lifetimne
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item = b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//=======================================================================================================

// Now lifetime annotations are type of generics so just like we generics we need to include 
// lifetime annotations inside of angle brackets 
struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn third_rule_explaination() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find the value");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}


// WRAP WHOLE THINGS
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn main () {

// }
