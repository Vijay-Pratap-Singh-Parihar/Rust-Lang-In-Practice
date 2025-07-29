// Closures are like function except they are anonymous meaning that they don't have names
// they could be stored as a variables and passed around in the parameters to a function and they capture a
// variables inside the scope in which they are defined in order to better understand closures we'll use them in the following

use std::{thread, time::Duration};

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);

    // Next Capturing the environment with closures
    capturing_environments_with_closures();
}

// Here we need to use generics and trait bounds, here we are using generic called T
// and below we have defined trait bound we are using fn short for function
// Without going deep Fn it is provided by a standard library and all closures implement one of the three
// Fn traits -> Fn, FnMut, FnOnce
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>
}

// We are implementing cacher structs
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    // In the below function to fix issue from the second turn we can utilize
    // hashmap as currently for the first time it will check if value exists if not then it proceed with
    // expensive operation and then save value to self.value then from next time calling it will check
    // then it will get first value and return every next operation will get first value to solve this
    // we can keep hashmap where arg will be key and value will be its calculation if earlier calculated else it calculate and save it to hashmap
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // If we see here we have not annotated the parameters for the closures, but in function it is important to mention parameters type
    // Because function are part of  explicit interface which is exposed to users thats why it is important to annotate the parameters and return type
    // but here it is optional we can make it explicit like this |num: u32| -> u32
    // let expensive_closure = |num| {
    //     println!("Calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut cached_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // example what closures do when we utilise it for the first time it takes its parameter and return type as a concrete type
    // let example_closure = |x| x;

    // let s = example_closure(String::from("Hello"));
    // let n = example_closure(5); // this will throw error as closure got the parameter as String because of earlier call

    if intensity < 25 {
        // here we could do expensive_closure call here only one time as a optimization but
        // instead of that we can do better by using memoization pattern by creating a struct which will hold our closure and the result of the closure
        println!(
            "Today, do {} pushups!",
            cached_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cached_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hyderated!");
        } else {
            println!(
                "Workout intensively, {}",
                cached_result.value(intensity)
            );
        }
    }
}

fn capturing_environments_with_closures() {
    // let x = 4;
    let x = vec![1, 2, 3];

    // Even though x is declared outside of our closure our closure still has access to x
    // because they are defined in a same scope 
    let equal_to_x = move |z| z == x;

    // Now what happened with function
    // It will throw error can't capture dynamic environment in a fn item, it will recommend us to use closure
    // as closure only can capture the dynamic environment
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }

    // Closures capture dynamic environment in three ways, which directly map to the three ways a function could take in input parameters
    // by taking ownership (FnOnce takes the ownership which are variables inside the closures environment, the once part of the name represents the fact that closures can't take ownership of the same variables more then once so these closures can be called once), 
    //by borrowing mutably(FnMut takes mutably borrows values), or by borrowing immutably (It Immutably borrows value ) 
    // Whenever we create a closure rust refers which of these traits to use based on how you use the values inside the closures environment we could however force the closure to take the ownership by using move keyword in front of the closure 
    // this is useful when you are passing a closure from one thread to another thread so you can also pass the ownership of the variable to another thread to the 
    // println!("Can't use x here because of we moved the x ownership: {:?}", x);

    // let y = 4;
    let y = vec![1, 2, 3];

    // Finally we are calling closures passing in y the to our closure
    assert!(equal_to_x(y));
}
