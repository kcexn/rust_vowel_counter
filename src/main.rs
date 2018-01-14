use std::env;

fn main() {
    let mut arguments = env::args();
    arguments.next(); 

    let text = match arguments.next() {
        Some(arg) => arg,
        None => String::from(""),
    };
 
    for character in text.as_str().chars(){
        println!("Match to vowels in the english language and map to a hash table");
        println!("{}", character);
    }
//    println!("{}", text);
}
