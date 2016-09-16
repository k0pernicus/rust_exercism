use std::ascii::AsciiExt;
use std::collections::HashMap;

pub fn is_pangram(sentence: &str) -> bool {

    let mut alphabet = HashMap::with_capacity(26);

    for c in b'a'..b'z' + 1 {
        alphabet.insert(c as char, 0);
    }

    let lower_sentence: String = sentence.to_lowercase();

    let iter = lower_sentence.chars().filter(|c| c.is_alphabetic()).filter(|c| c.is_ascii());

    for c in iter {
        if let Some(v) = alphabet.get_mut(&c) {
            *v += 1;
        }
    }

    for (_, v) in alphabet.iter() { 
        if *v == 0 {
            return false
        }
    }

    true

}
