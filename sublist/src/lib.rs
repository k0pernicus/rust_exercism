#[derive(PartialEq, Eq, Debug)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {

    if a.len() < b.len() {
        return false
    }

    if a.starts_with(b) {
        return true
    }

    return contains(&a[1..], b)
    
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        return Comparison::Equal
    }
    if a.len() < b.len() {
        if contains(b, a) {
            return Comparison::Sublist
        }
    } else {
        if contains(a, b) {
            return Comparison::Superlist
        }
    }
    return Comparison::Unequal
}
