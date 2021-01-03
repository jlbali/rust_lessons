


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}


        fn take_payment() {}
    }
}

fn serve_order() {}


mod back_of_house {

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast : String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;
//use self::front_of_house::hosting;  // Igual que arriba.

use std::fmt::Result;
use std::io::Result as IoResult;
use std::{io, cmp::Ordering}; // Nested Paths.

// O bien, importar hasta std::fmt y std::io y luego referenciar
// con fmt::Result o bien io::Result.



pub fn eat_at_restaurant() {
    // Two ways of calling the same function.
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    // Making advantage of the "use".
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("Quiero una tostada de {}", meal.toast);
    // No se puede acceder a seasonal_fruit porque es privado.

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}


/*
Using a semicolon after mod front_of_house rather than using a block
tells Rust to load the contents of the module from another file with the
same name as the module. To continue with our example and extract the
hosting module to its own file as well, we change src/front_of_house.rs to con-
tain only the declaration of the hosting module:
src/front_of_house.rs
pub mod hosting;
Then we create a src/front_of_house directory and a file src/front_of
_house/hosting.rs to contain the definitions made in the hosting module:
src/front_of_house/
hosting.rs
pub fn add_to_waitlist() {}
The module tree remains the same, and the function calls in eat_at
_restaurant will work without any modification, even though the definitions
live in different files. This technique lets you move modules to new files as
they grow in size.


*/





