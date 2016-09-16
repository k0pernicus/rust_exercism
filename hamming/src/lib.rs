pub fn hamming_distance(dna1: &str, dna2: &str) -> Result<u8, &'static str> {

    if dna1.len() != dna2.len() {
        return Result::Err("Lengths of strings are different")
    }

    Result::Ok(dna1.chars().zip(dna2.chars()).fold(0, |mut sum, (a_char, b_char)| if a_char != b_char {sum += 1; sum} else {sum}))

}
