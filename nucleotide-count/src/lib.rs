use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> usize {
    dna.chars().fold(0, |mut sum, c| if c == nucleotide {sum += 1; sum} else {sum})
}

pub fn nucleotide_counts(dna: &str) -> HashMap<char, usize> {
    
    let mut hash = HashMap::with_capacity(4);
    hash.insert('A', 0);
    hash.insert('C', 0);
    hash.insert('G', 0);
    hash.insert('T', 0);

    // CAREFUL!
    // Here, you canno't use functional syntactic sugar like `dna.chars().inspect(|c| match
    // hash.get_mut(&c) {...}`, because the body of inspect is unit, and Rust will not execute the
    // code until it create a new iterator!
    // Use for instead :-/
    for c in dna.chars() {
        if let Some(x) = hash.get_mut(&c) {
            *x += 1;
        }
    }

    hash

}
