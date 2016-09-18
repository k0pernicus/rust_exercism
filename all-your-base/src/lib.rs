///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
#[allow(unused_variables)]
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {

    if from_base == 0 || from_base == 1 || to_base == 0 || to_base == 1 {
        return Err(())
    }

    if number.len() == 0 {
        return Ok(Vec::new())
    }

    let mut base_10_number = 0;

    for (index, n) in number.iter().rev().enumerate() {
        if *n >= from_base {
            return Err(())
        }
        base_10_number += n * from_base.pow(index as u32);
    }

    let mut final_numbers : Vec<u32> = Vec::with_capacity(number.len());
    
    while base_10_number >= 1 {
        let current_division_f32 : f32 = (base_10_number / to_base) as f32;
        let current_division_u32 : u32 = current_division_f32.trunc() as u32;
        final_numbers.push(base_10_number % to_base);
        base_10_number = current_division_u32;
    }

    final_numbers.reverse();

    Ok(final_numbers)

}
