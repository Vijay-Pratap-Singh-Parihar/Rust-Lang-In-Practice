fn main() {
    // Two data types category scaler and compound
    /* Scaler types
        - Integer => 8-bit signed i8, unsigned u8; similary 16-bit, 32-bit, 64-bit, 128-bit, arch (usually depends on architecture 64bit or 32bit)
        - Floating-point numbers
        - Booleans
        - Character
    */
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)

    // Integer overflow: if we wrap up this greater then 255 in debug builds rust will panic and in release builds rust will perform
    // two's complement then 256 becomes 0 and 257 becomes 1, like language running server then you will change to 256 you will get
    // get an error warning you of the overflow
    let f: u8 = 255;


    // addition
    let sum: i32 = 5 + 10;
    // subtraction
    let difference: f64 = 95.5 - 4.3;
    // multiplication
    let product: i32 = 4 * 32;
    // division
    let quotient: f64 = 56.6 / 21.2;
    // remainder
    let remainder: i32 = 34 % 5;

    // Boolean
    let t: bool = true;
    let f: bool = false;

    // Character
    let c: char = "z";
    let z: char = "Z";
    let heart_eyed_cat: char = "F";

    /* Compound types
        which represent group of values
        tuple type which you can think about as fixed size array of related data that could be of different type
    */
    let tup: (&str, i32) = ("Let's get rusty", 199_999);
    // destructuring tuple
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    // Arrays are also a type of compound type
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    let byte = [0; 8]; // It says 8 values all set to 0 

}