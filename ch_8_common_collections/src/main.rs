use unicode_segmentation::UnicodeSegmentation;

// Common collection: Collection allow you to store multiple values but unlike arrays or tuples, collection are allocated on heap meaning the size of the collection could grow or shrink as needed
fn main() {
    vectors_exp();
    string_exp();
    hashmap_exp();
}

// Specifically we'll talk about vectors , strings and hash maps 
fn vectors_exp() {
    let a = [1, 2 ,3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // This looks very similar to array initialization this is initialize with vec !-> macros 
    let mut v2 = vec![1, 2, 3];
    // v2 is available in its scope only like if { v2 } scope will be curly bracket for this

    // Now how to access index digit
    // we cannot have mutable references and immutable references at the same time
    let third = &v[2];
    v2.push(6);
    println!("The third elementis {} and its length is: {}", third, v2.len());
    // let invalid_accessesing = &v2[20]; // This will throw error index is out of bound the len is 3

    // We can manage invalid access like above we were trying to access the element using MATCH in which if that index found it will execute option enum some part else it will print none part
    // Here we will be using option enum which we have discussed will be some value or no value means none
    // If you want to handle out of bound index error without crashing your program we can manage with this
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    iterating_over_vectors();
}

fn iterating_over_vectors() {
    let mut v = vec![1, 2, 3, 4, 5];

    // For in loop
    for i in &mut v { // for modifing the value take &mut v mutable reference of v else take &v immutable reference of v vector
        *i += 50; // Here we will be using * dereferencing operator to get the underlying value and add to it 
        println!("Element: {}", i);
    }

    storing_enums_to_vector();
}

// Storing enum values under the vector
fn storing_enums_to_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer")
    }
}

// =================================================================================================
// Strings are pretty complicated in higher programming the complexity of strings is abstracted away from the programmer but in lower programming languages such as rust
//  we have to deal with that complexity so with that preface, In rust strings are stored as a utf-8 encoded bytes we see a string as letters and numbers
// but a computer only understands ones and zeros, so in memory list is just list or collection of ones and zeros... In order to understand utf-8 we have to understand ASCII then unicode then 

fn string_exp() {
    // String are stored as a collection of UTF-8 encoded bytes
    let s1: String = String::new(); // We can create new empty string
    let s2: &str = "initial contents"; // We can create directly string slices
    let s3: String = s2.to_string(); // We can convert string slice to string using to_string method
    let s4: String = String::from("initial contents"); // We can create own string by using string slice to from function


    // Just like vector string can grow and shrink in size here we have an example
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    // String will be foobar!

    // Concatination and moving ownership for example below after after str3 we cannot access str1 as we have moved str1 ownership
    let str1 = String::from("Hello, ");
    let str2 = String::from("world!");
    let str3 = str1 + &str2; // Moving s1 ownership from str1 to str3 and appending str2 to str3 by using + and passing by reference

    // How to access first element
    let hello: String = String::from("jqwenjwqe");
    // let c: char = hello[0]; // we cannot access element like this as we know string character can range from 1 byte to 4 bytes

    // To better understand of about accessing the element why we cant do this, to access there are three relevant ways a word
    let hello:String = String::from("नमस्ते");

    // Bytes they are 18 bytes in this word where we access one byte like this hello[0], To accessing bytes we have separate method
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141,.. ]
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Scalar values
    // ['न', 'म', 'स', '्', 'त', 'े']
    for c in "नमस्ते".chars(){
        println!("{}", c);
    }

    // Grapheme clusters It is not default iterable as we have to import unicode-segmentation
    // ["न", "म", "स्ते"]
    for d in "नमस्ते".graphemes(true) { // here we are passing true to get extended grapheme clusters which we want
        println!("{}", d);
    }
    // So rust not able to what we want to receive bytes, scalar value or grapheme clusters 
}

//=================================================================================================================================
// HashMaps: It allow us to store key value pairs and key-value could be of any types also it uses hashing function to determine how to place those keys
// In order to create hashMap we need to bring hashmap type into scope from standard library
use std::collections::HashMap;

fn hashmap_exp() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    // Similarly hashmap is created like vector and strings we can use new function
    let mut scores = HashMap::new();

    // To add entries in hashmap we use insert functions and then we specify
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // Like Above I am not passing the reference of blue or yellow variable I am moving the ownership of those variables
    // similary If I am accessing that variable it will throw error which will state that ownership is moved
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // This is how we can get value of key here it will return Option enum because if the respective key not found it will return none else it will break whole code

    // This is how we can iterate over scores hashmap and return key and value printed
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    more_on_hashmaps();
}


fn more_on_hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // This will overwrite the value 10 which was set earlier

    // To avoid overwritten use or_insert like below
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);

    // Now lets see a value in our hashmap based on an old value here we have an string, now we want to do is to populate our hashmap with the word count in our string so
    let text = "Hello world wonderful world"; // hello will be a key and present 1 time in string as a value hello: 1, world will be a key and it is 2 times present world: 2

    let mut map = HashMap::new();

    for word in text.split_whitespace() { // ["hello", "world", "wonderful", "world"]
        let count = map.entry(word).or_insert(0); // What this will do it will check if that word is present then do nothing if not then it will add one, but our or_insert function returns a mutable reference to our value
        // So now we have the mutable reference of that then we can dereference it and increment it
        *count +=1;
    }

    println!("{:?}", map);
}
