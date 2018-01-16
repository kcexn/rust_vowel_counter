pub struct VowelCount {
    pub a: u32,
    pub e: u32,
    pub i: u32,
    pub o: u32,
    pub u: u32,
}

impl VowelCount {
    pub fn new(a: u32, e: u32, i: u32, o: u32, u: u32) -> VowelCount {
        VowelCount{
            a,
            e,
            i,
            o,
            u,
        }
    }
}


pub fn count(text: &str) -> ( u32, VowelCount ) {
    
    let mut count = 0;
    // initialise everything to 0.
    let mut counter = VowelCount::new(0,0,0,0,0); 
//    println!("{}, {}, {}, {}, {}", counter.a, counter.e, counter.i, counter.o, counter.u);
//
    for character in text.to_lowercase().chars() {
        match character {
            'a' => { 
                count = count + 1;
                counter.a += 1
            },
            'e' => {
                count = count + 1;
                counter.e += 1
            },
            'i' => { 
                count = count + 1;
                counter.i += 1
            },
            'o' => {
                count = count + 1;
                counter.o += 1
            },
            'u' => { 
                count = count + 1;
                counter.u += 1
            },
            _ => continue,
        }
    }

    (count, counter) 
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        return true
    }
}
