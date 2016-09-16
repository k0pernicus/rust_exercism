pub fn square_of_sum(number: u32) -> u32 {

    let nsum: u32 = (1..number + 1).sum();
    nsum * nsum

}

pub fn sum_of_squares(number: u32) -> u32 {

    (1..number + 1).map(|n| n * n).sum()

}

pub fn difference(number: u32) -> u32 {

    square_of_sum(number) - sum_of_squares(number)

}
