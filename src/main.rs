extern crate vowel_utils;

use std::env;

fn main() {
    let mut arguments = env::args();
    arguments.next(); 

    let text = match arguments.next() {
        Some(arg) => arg,
        None => {
            println!("no arguments received.");
            return
        },
    };

    let vowel_amount = vowel_utils::count(&text);
    println!("{}", vowel_amount);
}
