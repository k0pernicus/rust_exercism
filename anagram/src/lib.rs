fn return_sort_char_list<'a>(string: &'a str) -> Vec<char> {
   let mut string_chars: Vec<char> = string.chars().collect::<Vec<char>>();
   string_chars.sort();
   string_chars
}

pub fn anagrams_for<'a>(string: &'a str, candidates: &[&'a str]) -> Vec<&'a str> {
    let lstring = string.to_lowercase();
    let string_chars = return_sort_char_list(&lstring);
    let mut final_candidates: Vec<&'a str> = Vec::new();
    for &c in candidates.iter() {
        let lc = c.to_lowercase();
        if return_sort_char_list(&lc) == string_chars && lstring != lc {
            final_candidates.push(c);
        }
    }
    final_candidates
}
