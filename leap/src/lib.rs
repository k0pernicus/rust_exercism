pub fn is_leap_year(year: u32) -> bool {

    match (year % 400, year % 4, year % 100) {
        (0, _, _) => true,
        (_, 0, e) => if e != 0 {
            true
        } else {
            false
        },
        _ => false     
    }

}
