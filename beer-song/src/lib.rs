fn get_string(number: u8) -> String {
    match number {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", number, number, number - 1),
    }
}

pub fn verse(number: u8) -> String {
    return get_string(number)
}

pub fn sing(number1: u8, number2: u8) -> String {
    let mut string_to_return = String::with_capacity(100);
    for number in (number2..number1 + 1).rev() {
        string_to_return.push_str(&get_string(number));
        if number != number2 { 
            string_to_return.push_str("\n");
        }
    }
    string_to_return
}
