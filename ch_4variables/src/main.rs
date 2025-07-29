fn main() {
    // calculating_length_using_immutable_references();
    // change_or_append_to_string_using_mutable_references();
    // mutable_immutable_references_and_its_scope();
    // dangling_main();
    slices_main();
}

fn calculating_length_using_immutable_references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // When we are passing s1 directly and access as a parameter in function directly String
    // to solve that we have passed by reference by using &s1 as arguments and &String type variable in perameters
    change_or_append_to_string_using_mutable_references();
    mutable_immutable_references_and_its_scope();
}

fn calculate_length(s: &String) -> usize {
    let length: usize = s.len(); // len() returns the length of a string
    length
}


/*
______s_______      _______s1_______     | index | value |
|name | value|     | ptr    |------|---> |  0    |  h    |
| ptr |------|---->| len    |  5   |     |  1    |  e    |
|_____|______|     |capacity|  5   |     |  2    |  l    |
                   |________|______|     |  3    |  l    |
                                         |  4    |  o    |                    
                                         |_______|_______|
Passing references as function parameters is called borrowing because we
    are borrowing the value we are not actually ownership of it also note that references are immutable
    by default so if we try to modify we will get error cannot borrow `*s` as mutable, as it is behind a `&` reference `s`
    is a `&` reference, so the data it refers to cannot be borrowed as mutable. to achive that below is an example
*/  

// ==============================================================================================================================================

fn change_or_append_to_string_using_mutable_references() {
    let mut s1: String = String::from("hello");
    change(&mut s1);
    println!("{s1}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ===================================================================================================================================================

// Data Race issue: If two pointers are pointing same data and one of the pointer is used to write to the data and there's no
    // mechanism to synchronize data access between those pointers in that situation we can image one is reading the data another
    // Pointer is modifing the data in that case we will get corrupt data back to solve this we can pass immutable refrences
fn mutable_immutable_references_and_its_scope() {
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s; // for above

    let r1 = &s;
    let r2 = &s;// Now there will be no errors, we have to remove mutable from variable declaration
    // Now what happen if we mix immutable references with mutable references
    // let r3 = &mut s; // add mut to variable declaration, here we will get error that you cannot have mutable references if you already have immutable references 

    println!("{}, {}", r1, r2); // But we can create after this utilizing mutable reference without an error as r1 and r2 scope ends here because it is used lastly here rust will clear its memory after this

    let r3 = &mut s;
    println!("{}", r3);
}

// ===========================================================================================================================================================

// Dangling references: what happend when variable is referencing invalid data
// fn dangling_main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     // After this scope executing s value will be dropped and then further it will be utilized
//     // it reference is return which will be referencing same address where s was existing but now it will be invalid
//     &s
// }

/*
    The Rules of References
    1. At any given time, you can have either one mutable reference or
    any number of immutable references.
    2. References must always be valid.

    // We pass reference of string or any variable when we don't want to take its ownership
*/

// =============================================================================================================================================================

fn slices_main() {
    let mut s = String::from("hello world");
    // String Slice
    // let hello: &str = &s[..5]; // Its a pointer which is pointing 0 to length of 5 of string s
    // let world: &str = &s[6..]; // Its a pointer which is pointing 6 to last element of string s
    let s2 = "Hello World";
    let word = first_word(s2);
    s.clear(); // this will make string to empty string but still word will hold first word length
    println!("word {}", word);

    // In array of integers
    let a = [1, 3, 5, 6];
    let slice = &a[0..2];
    println!("Slice of integer array: {:?}", slice);
}

fn first_word(s: &str) -> &str { // for index its return type is this usize
    let bytes = s.as_bytes(); // we take the string and convert it to bytes

    for (i, &item) in bytes.iter().enumerate() {
        /* 
            Here i is index of element and &item is value of element it self this is done by enumerate() is a method available on any type that implements the Iterator trait. It transforms an iterator into a new iterator that yields pairs of (usize, T), where usize is the zero-based index and T is the item from the original iterator. 
            iter() is a method commonly found on collections (like Vec, [T], HashMap, HashSet, etc.) that allows you to create an iterator over the elements of that collection
        
            item == b' ':

            item: This variable represents the current byte being processed in the iteration. In Rust, when you iterate over a &[u8], each item will be of type u8 (an unsigned 8-bit integer, representing a single byte).

            b' ': This is a byte literal in Rust. It represents the ASCII byte value for a space character (     ). Using b'' is explicit that you're dealing with a single byte, not a char (which can be a multi-byte Unicode scalar value).

            ==: This is the equality comparison operator. So, item == b' ' checks if the current byte (item) is equal to the byte value of a space.
        */
        if item == b' ' {
            // return i; // if any space found it will return its index which means "vijay pratap" so it will return 0-5 means 5th index it will have space and return length 
            return &s[0..i];
        }
    }

    // s.len()
    &s[..]
}
