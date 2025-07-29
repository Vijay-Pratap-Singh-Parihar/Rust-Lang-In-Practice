/* Module can contain multiple modules in it, they can contain structs enums constants traits and so on,
    Structuring our code this way keeps it clean and readable

    This is the module tree for the code below:
    crate
     |__front_of_house
         |__hosting
         |   |__add_to_waitlist
         |   |__seat_at_table
         |__serving
            |__take_order
            |__serve_order
            |__take_payment
        
*/
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {};
    
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

//=====================================================================
// PATHs: If you wanted to reference a file inside your directory tree, you need to specify a path to that file in the same way you want to refer an item
// in your module tree lets say a function, you need to specify a path to that function 

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); 
    // Absoulute path always start from crate as it take crate as a current directory as parent directory
    // If we will not specify hosting as pub(public) it will throw an error as hosting cannot be accessable as it is private
    // After we make hosting as pub(public) then add_to_waitlist will start showing same error as it is a child which is also private until we make it public using pub above

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

//===================================================================================
// Relative PATHs using the super keywords
fn serve_order() {

}
mode back_of_house {
    fn fix_incorrect_order() {
        cook_order(); // Here we are able to call this because it is defined in same module
        super::serve_order(): // We are also able to call serve order function using relative path using super keyword which allow us to reference the parent module which in this case is crate
    }

    fn cook_order() {}
}

// =====================================================================================
// STRUCTS: Privacy rules when it comes to struct using this example
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // It will be through error that Breakfast and summer are private by default we have to make Breakfast struct as public and same summer function also public using pub keyword
    let mut meal = back_of_house::Breakfast::summer("Some Food");

    // Now I want to modify my meal then I will modify the toast which is declared above under Breakfast struct
    // It will throw error as by default struct fields are private we have to make them public too, then we are allowed to assign a value
    meal.toast = String::from("Wheat");

    // Similary If we try to modify seasonal_fruit field of Breakfast struct we will get same error as field seasonal_fruit of struct breakfast is private field

}

// ==============================================================================================================================\
// ENUMS: Privacy rules when it comes to enums
mod back_of_house {
    enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup; //It will show an error that need to covert enum Appetizer to public as it is private, as making appetizer public will allow its variants or fields are public as well
    let order2 = back_of_house::Appetizer::Salad;
}

// ===================================================================================================================================
// USE: the use keywords allow us to use as whole path in short form as last suffix
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// If we want to use somewhere else below hosting like outside this then we have to assign pub(public) to below command to get accessed hosting from outside
pub use crate::front_of_house::hosting;// Here we have specified as a absolute path here we can replace crate with self for converting it in relative path

pub fn eat_at_restaurant() {
    // this add to waitlist function is not a local function, on the other hand if you're bringing enums structs or other into scope its idiomatic to specify the full path the exception to this is if you're bringing
    //  two items lets say two structs from different modules into the scope and they have the same name in that case we have to bring parent module into the scope so the names don't conflict NEXT EXAMPLE IS FOR THIS ONLY
    hosting::add_to_waitlist();// Writing this multiple time is way easier then writing 
    hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist(); // this as it is long enough to write multiple times
}

//==============================================================================================================
// Conflicting names to bring there parents to the scope
// use std::fmt; // we cannot bring directly Result into our scope from fmt as it will conflict with io Result that is why we are importing there parent
// use std::io;

// Another type is to change its name for the scope
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

//=========================================================================================================================
// We want to use external module here we will first add it to toml file under dependencies with there version if its from registry
// use rand::Rng;// Now what we want to bring some more code from rand into the scope
// use rand::ErrorKind::Transient;
// use rand::CryptoRng; // we could list here like this but we can refactor this into one line using nested paths
use rand::{Rng, CryptoRng, ErrorKind::Transient};
// Similarly if we are bring io then we can gather its common like till which parent it is common then rest will be into braces
use std::io::{self, Write};
//Instead of this
// use std::io::self;
// use std::io::Write;
// Like this we can bring all the public items under std::io into the scopr
// std::io::*;


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}

// Now What If everything gets too messy like multiple modules into single file for that we create separate file for per module and include here like below
// everything will work as expeted
mod front_of_house;
// Above put the content in a file with the same name as module, in this case we declare front_of_house above and its content which is refered is front_of_house.rs file,
// Similarly any child module under front_of_house.rs will be created under a directory name will be same of the file and then under that pub modules with the name of module file is created

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}