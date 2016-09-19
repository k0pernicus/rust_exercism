use std::collections::HashMap;

static USED_BRACKETS: [char; 6] = ['(', ')', '[', ']', '{', '}'];

pub struct Brackets {
    next_b: HashMap<char, char>,
    suite: String
}

impl<'a> From<&'a str> for Brackets {

    fn from(string: &str) -> Self {
        Brackets::new(string)
    }

}

impl Brackets {

    fn new(string: &str) -> Self {
        let mut h: HashMap<char, char> = HashMap::with_capacity(3);
        h.insert('(', ')');
        h.insert('[', ']');
        h.insert('{', '}');
        Brackets {
            suite: string.chars().filter(|c| USED_BRACKETS.contains(c)).collect::<String>(),
            next_b: h,
        }
    }

    pub fn are_balanced(&self) -> bool {
        let mut vec_chars: Vec<char> = Vec::with_capacity(self.suite.len());
        for c in self.suite.chars() {
            println!("{}", c);
            if c == '(' || c == '[' || c == '{' {
                vec_chars.push(c);
            } else {
                if vec_chars.is_empty() {
                    return false
                }
                let current_char: char = vec_chars.pop().unwrap();
                println!("Current: {}", current_char);
                if let Some(c2) = self.next_b.get(&current_char) {
                    if c != *c2 {
                        return false
                    }
                } else {
                    return false
                }
            }
        }
        if !vec_chars.is_empty() {
            return false
        }
        return true
    }
}
