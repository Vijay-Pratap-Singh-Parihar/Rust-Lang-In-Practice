fn main() {
    let sum = my_function(11, 22)
    println!("The sum is: {}", sum);
}

/*
In rust it is compulsary to use snake case for the naming of function
else
    fn myFunction() {
  |    ^^^^^^^^^^ help: convert the identifier to snake case: `my_function`
*/ 
fn my_function(x: i32, y: i32) -> i32{
    println!("Another function!!");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // multiple way to return values below and above we have to mention return type -> i32
    x + y
    // let sum = x + y;
    // sum
    // return sum;
}