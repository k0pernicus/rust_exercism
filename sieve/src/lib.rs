use std::collections::HashMap;

pub fn primes_up_to(limit: usize) -> Vec<usize> {
    
    let possible_primes : Vec<usize> = (2..limit + 1).collect();

    let mut hash : HashMap<usize, bool> = HashMap::with_capacity(limit); 

    for i in possible_primes.iter() {
        let mut index = 2;
        loop {
            let current_index = index * i;
            if current_index <= limit {
                *hash.entry(current_index).or_insert(false);
            } else {
                break;
            }
            index += 1;
        }
    }

    possible_primes.into_iter().filter(|u| !hash.contains_key(u)).collect()

}
