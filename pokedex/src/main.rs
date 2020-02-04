mod front;
mod back;
mod structs;

use std::collections::HashMap;
pub use crate::front::listening_loop::listen as run;
pub use crate::structs::Request;

fn main() {
    let empty_request = Request {
        dictionary: HashMap::new(),
        raw: String::from(""),
        parsed: [String::from(""), String::from(""), String::from("")]
    };

    println!("HELLO WELCOME TO THE POKEDEX (Pas du tout)");
    run(empty_request);
}
