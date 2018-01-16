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

    let (vowel_amount, counter) = vowel_utils::count(&text);
    println!("there are {} vowels", vowel_amount);
    println!("a: {}, e: {}, i: {}, o: {}, u: {}",
             counter.a,
             counter.e,
             counter.i,
             counter.o,
             counter.u);
}
