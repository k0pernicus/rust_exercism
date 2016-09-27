extern crate rand;

use rand::{thread_rng, Rng};

fn generate_new_name() -> String {
    let mut new_name = String::new();
    new_name.push(thread_rng().gen_range(b'A', b'Z') as char);
    new_name.push(thread_rng().gen_range(b'A', b'Z') as char);
    for _ in 0..3 {
        new_name.push(thread_rng().gen_range(b'0', b'9') as char);
    }
    new_name
}

pub struct Robot {
    name: String,
}

impl Robot {
    
    pub fn new() -> Self {
        Robot {
            name: generate_new_name(),
        }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name[..]
    }

    pub fn reset_name(&mut self) {
        self.name = generate_new_name();
    }

}
