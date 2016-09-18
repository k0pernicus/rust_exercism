use std::fmt;

static numerus_to_roman: [(usize, &'static str); 14] =
    [(1, "I"),
     (4, "IV"),
     (5, "V"),
     (6, "VI"),
     (9, "IX"),
     (10, "X"),
     (40, "XL"),
     (50, "L"),
     (90, "XC"),
     (100, "C"),
     (400, "CD"),
     (500, "D"),
     (900, "CM"),
     (1000, "M")];

pub struct Roman {
    num: usize
}

impl From<usize> for Roman {

    fn from(num: usize) -> Roman {
        Roman::new(num)
    }

}

impl fmt::Display for Roman {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut start = self.num.clone();
        let mut roman_result = "".to_string();
        for &(num, roman_f) in numerus_to_roman.iter().rev() {
            loop {
                if num > start {
                    break;
                } else {
                    roman_result.push_str(roman_f);
                    start -= num;
                }
            }
        }
        write!(f, "{}", roman_result)
    }

}

impl Roman {

    pub fn new(num: usize) -> Roman {
        Roman { num: num }
    }

}
