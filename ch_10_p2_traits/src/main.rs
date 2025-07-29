// Shared behavior using traits
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

// Implementing summary trait for news article type
// For sample removing implementation from here as it will utilize default implementation
impl Summary for NewsArticle {
    summarize_auhtor(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content) // Here we are overwritten the default implementation
    }
}

/* In this we can find shared behavior between a tweet and news article which in this case summarization
- to more specific by shared behavior is a methods so traits allow us to define a set of methods
- which are shared across different types, inside a curly braces we are going to have method which is shared among others
- We only define method we dont specify its body and that because we dont dictate implementation we just wanna say
 that for every type which implements this trait they should have this method
- By default we can add on implementations like from this
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    to below one
 */
pub trait Summary {
    // fn summarize(&self) -> String {
    //     String::from("Read more...")
    // }
    // Instead of having one function to summarize we will be having two functions

    fn summarize_author(&self) -> String; // In NewsArticle and Tweet we have to define default implementation

    fn summarize(&self) -> String {
        format!("(Read more from {}...}", self.summarize_auhtor())
    }
}


// Trait Bounds
// Lets now understand traits as a parameters, below I have a function called notify it takes item which is reference of something and that something is Summary
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait Bounds example: This is the longer form of the function used above using the impl syntax and input could be used for more concise code in simple cases
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }


// Trait Bound more complex example earlier we were using one parameter and now we will be using two parameters
// Here I want both which implements summary and display
// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
//     //...
// }

// // We could also do that for trade bound syntax after summary we just add Display and this is saying we have type T which has to implement the summary
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//    // ,,,
// }


// // One last thing to know here specifying multiple trait could hinder readability
// // For example we have some_function here which handles T and U types in which T holds Display and Clone and U hold 
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     //,...
// }

// // above trait bounds can be written as below for easy readability
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {
//     //....
// }

// Return types: Here we function which returns summarizable we can see below return type is impl summary
// Here we return something is summary trait not a concrete type now under function body we return a tweet
// then in main function we print line with summarize function
// There is some restriction like below we have returned tweet directly, but we cannot return multiple type of imp in that function
// for example if I take a parameter as bool true or false on which we will decide that if we wanna return NewsArticle or Tweet that is not allowed as our compiler implements the infiltrate syntax we'll learn
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The Sky is not actually falling."),
    };

    // println!("Tweet Summary: {}", tweet.summarize());
    // println!("Article Summary: {}", article.summarize());

    // Trait Bounds function is called
    // notify(&article);

    // Return type as a impl summary
    println!("{}", returns_summarizable().summarize());

    trait_balance();
}

// Trait balance: Conditionally Implement Methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

// This Implementation is the Pair Implementation with type T which is generic, means this impl block is for any pair struct
// under impl block we have new function which creates a new Pair
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Another impl block which says type T has to implement display and partial order then we define this
// compare method which compares x and y and we can do that because we know x and y implement display and partial order trait 
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//=================================================================================================
// Blanket Implementation : Basically we can implement a trait on a type that implements another trait
// Here we are implementing ToString trait for type any T that implements Display trait it is widely used library in standard
impl<T:Display> ToString for T {
    // --snip--
}


// Generics which let us abstract concrete types in order to reduce duplication
// Traits allow us shared behavior across multiple types
// Traits bound are created using generics with traits which allow us to instead of concrete types use a generic type that specifies 