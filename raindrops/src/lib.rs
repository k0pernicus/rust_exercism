fn is_pling(number: u32) -> bool {
    (number % 3) == 0
}

fn is_plang(number: u32) -> bool {
    (number % 5) == 0
}

fn is_plong(number: u32) -> bool {
    (number % 7) == 0
}

pub fn raindrops(number: u32) -> String {
    let mut string_to_return = String::with_capacity(5);
    if is_pling(number) {
        string_to_return.push_str("Pling");
    }
    if is_plang(number) {
        string_to_return.push_str("Plang");
    }
    if is_plong(number) {
        string_to_return.push_str("Plong");
    }
    if !string_to_return.is_empty() {
        string_to_return 
    } else {
        number.to_string()
    }
}
