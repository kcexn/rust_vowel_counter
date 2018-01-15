
struct VowelCount {
    a: u32,
    e: u32,
    i: u32,
    o: u32,
    u: u32,
}

pub fn count(text: &str) -> u32 {
    
    let mut count = 0;
     
    for character in text.chars() {
        match character {
            'a' => count = count + 1,
            'e' => count = count + 1,
            'i' => count = count + 1,
            'o' => count = count + 1,
            'u' => count = count + 1,
            _ => continue,
        }
    }

    count 
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        return true
    }
}
