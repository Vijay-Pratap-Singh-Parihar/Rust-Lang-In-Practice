fn main() {
    panic_exp();
    result_enum_exp();
}

//==========================================================================================

fn panic_exp() {
    // panic!("Crash and burn");
    // For example for panic, to check what was the flow and where it breaks we can run command again using environment variable named and command is this: RUST_BACKTRACE=1 cargo run
    a();
}

fn a() {
    b();
}

fn b() {
    c(23);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!!");
    }
}

// =========================================================================================
// There are some errors which are recoverable error that we can handle gracefully and don't want to crash our programs those cases we have the result enum it is similar to option enum
// Option enum returns some value or none value here Result enum return success or error: Ok returns some generic success value and Err returns failure value
// fn result_enum_exp() {
//     enum Result<T, E> {
//         Ok(T),
//         Err(E)
//     }
// }

use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn result_enum_exp() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // This will be executed if error is found which is kind of NotFound
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem openning the file: {:?}", other_error)
            }
        },
    };

    // Exact same this is can be implemented using clousers
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    more_on_unwrap();
}

fn more_on_unwrap() {
    // ERROR PROPOGATION
    // Here unwrap do the exact same this which is accomplised using match below
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error);
    // };

    read_username_form_file();
}

fn read_username_form_file() -> Result<String, io::Error> {
    // Every thing now in written in a consise way where empty string is created then it will open a file if any issue in opening the file occurs then ? it will throw an error and return from here
    // After that if it suceeds then it will call read_to_string function with s then if it fails then it will throw error and return from here else it will succeed and continue with Ok(s) success message
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    println!("Hello {}", s);
    Ok(s)
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };

    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s);
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // };

    making_code_more_concise();
    operator_and_its_usage();
}

// before moving to this start with modifying import of fs::File to fs::{self, File} above
fn making_code_more_concise() -> Result<String, std::io::Error> {
    // fs module has the function of read_to_string for the path which was given below it will return the result or it will return error 
    fs::read_to_string("hello.txt")
}
// ? operator is only utilized when the function where it is added as a suffix which function returns any result
fn operator_and_its_usage() -> Result<(), Box<dyn Error>> {// here in success we return () and in error we return a trait which we will go through at chapter 17
    let f = File::open("hello.txt")?;
    Ok(())
}


/*
 What should we use panic macro or result enum and when? the answer is by default we will
 be using result enum and error propagation this prevents the program from crashing and air propagation allows
 the caller to decide how best to handle the error in exceptional circumstances in circumstances

 Panic macro should be used in that circumstances the error is not possible to recover and your program
 cannot continue because it is in a bad state so you must end the program, appropriate place to allow your code 
*/

