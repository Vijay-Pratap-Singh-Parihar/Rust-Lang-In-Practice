// // Module is of type which uses keyword mod and then module name then curly braces
// // modules inside of them they could also contain structs, enums, constants, traits and so on, structuring our code this way keeps organised if in the future for example to add the ability 
// /*
//     Package:        The entire project directory with package.json	         ->   Aapka poora project folder with Cargo.toml.
//     Crate:          The main file(APP) (index.js) OR a published npm package	     ->  Aapke code ka final product: ya toh ek app ya ek library.
//     Module (mod):	Splitting code into different files (user.js, auth.js)	 ->   Code ko organize karne ke liye chote-chote groups/files.
//     Path & use:	    require() or import/export	                             ->   Ek module se doosre module mein code use karne ka tareeka.
// */
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {

//         }
//         fn seat_at_table() {

//         }
//     }

//     mod serving {
//         fn take_over() {

//         }

//         fn serve_over() {

//         }

//         fn take_payment() {

//         }
//     }
// }


// src/lib.rs (Your core backend logic)
pub mod user_service { // This module will contain user related functions
    pub fn create_user(data: String) { // Public function
        println!("Creating user with data: {}", data);
        // ... database logic, etc.
    }

    pub fn get_user(id: u32) -> String { // Public function
        println!("Getting user with ID: {}", id);
        // ... database lookup
        format!("User with ID: {}", id)
    }
}

// ... other modules for other services, models, etc.