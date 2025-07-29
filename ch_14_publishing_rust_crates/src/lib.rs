// below is used to for items containing the comment
//! # My Crate
//! 
//! 'my_crate' is a collection of utilities to make performing certain
//! calculations more convenient

// Below contains item following the comment in document
/// Adds one to the number given,
/// 
/// # Examples
/// 
/// ```
/// let arg = 5
/// let answer = my_crates::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
// We can utilize to check how it will look like in documentation by running the command 
// Command: cargo doc --open
// We can also check it will run like test it will take values form above example then run the below function
pub fn add_one(x: i32) -> i32 {
    x + 1
}