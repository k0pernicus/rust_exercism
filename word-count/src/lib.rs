use std::collections::HashMap;

pub fn word_count(string: &str) -> HashMap<String, u32> {

    let mut hash = HashMap::new();

    // If the string contains non alphanumeric values, replace them all with spaces!
    let clean_string : String = string.to_lowercase().chars().map(|c| if c.is_alphanumeric() || c.is_whitespace() { c } else { ' ' }).collect();

    for word in clean_string.split_whitespace() {
        let mut occurence = hash.entry(word.to_string()).or_insert(0);
        *occurence += 1;
    }

    hash

}
