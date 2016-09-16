fn get_score(c: char) -> u8 {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => 0
    }
}

pub fn score(word: &str) -> u8 {
   word.to_lowercase().chars().fold(0, |mut sum, c| { sum += get_score(c); sum } )
}
