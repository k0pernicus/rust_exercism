fn return_phone_number(phone_number: &str) -> Option<String> {
    if phone_number.is_empty() {
        return None
    }
    let digits_number: String = phone_number.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let digits_number_len: usize = digits_number.len();
    if digits_number_len < 10 || digits_number_len > 12 || (digits_number_len == 11 && !digits_number.starts_with("1")) {
        return None
    } else {
        if digits_number_len == 11 {
            return Some(digits_number[1..].to_string())
        }
        return Some(digits_number)
    }
}

pub fn number(phone_number: &str) -> Option<String> {
    return_phone_number(phone_number)
}

pub fn area_code(phone_number: &str) -> Option<String> {
    match return_phone_number(phone_number) {
        Some(phone_number) => Some(phone_number[..3].to_string()),
        _ => None
    }
}

pub fn pretty_print(phone_number: &str) -> String {
    match return_phone_number(phone_number) {
        Some(phone_number) => format!("({}) {}-{}", &phone_number[..3], &phone_number[3..6], &phone_number[6..]),
        _ => "invalid".to_string()
    }
}
