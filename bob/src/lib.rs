pub fn reply(msg: &str) -> String {
    if msg.is_empty() {
        return "Fine. Be that way!".to_string()
    }
    if msg.ends_with("?") {
        return "Sure.".to_string()
    }
    if msg.to_uppercase() == msg {
        return "Whoa, chill out!".to_string()
    }
    "Whatever.".to_string()
}
