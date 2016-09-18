use std::collections::BTreeMap;

pub struct School {
    
    list: BTreeMap<u32, Vec<String>>

}

impl School {

    pub fn new() -> School {
        School { list: BTreeMap::new() }
    }

    pub fn add(&mut self, the_grade: u32, the_name: &str) {
        let mut the_entry = self.list.entry(the_grade).or_insert(Vec::new());
        the_entry.push(the_name.to_string());
        the_entry.sort();
    }

    pub fn grade(&self, the_grade: u32) -> Option<&Vec<String>> {
        self.list.get(&the_grade)
    }

    pub fn grades(&self) -> Vec<u32> {
        self.list.keys().map(|u| *u).collect()
    }

}
