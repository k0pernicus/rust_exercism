// Generic structure that contains a single and specific data type.
#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    content: Vec<T>,
}

impl<T: Clone + Copy + Ord + PartialEq> CustomSet<T> {

    pub fn new(vector: Vec<T>) -> Self {
        let mut new_vector = vector;
        new_vector.sort();
        CustomSet {
            content: new_vector,
        }
    }

    pub fn add(&mut self, element: T) {
        // Don't add the same element twice in the `Set`
        if self.contains(&element) {
            return
        }
        self.content.push(element);
        self.content.sort();
    }

    pub fn contains(&self, element: &T) -> bool {
       self.content.contains(element) 
    }

    pub fn is_subset(&self, other_set: &CustomSet<T>) -> bool {
        if self.content.len() > other_set.content.len() {
            return false
        }
        // Get a reference for each element in `content`, and try to
        // determine if each of these elements is contained in
        // `other_set`
        self.content.iter().all(|ref x| other_set.contains(&x))
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn is_disjoint(&self, other_set: &CustomSet<T>) -> bool {
        // If at least one of these sets is empty, you can return `true` 
        if self.is_empty() || other_set.is_empty() {
            return true
        }
        self.content.iter().all(|ref x| !other_set.contains(&x))
    }

    pub fn difference(&self, other_set: &CustomSet<T>) -> CustomSet<T> {
        let mut difference_content = self.content.clone();
        difference_content.retain(|x| !other_set.contains(&x));
        CustomSet {
            content: difference_content
        }
    }

    pub fn intersection(&self, other_set: &CustomSet<T>) -> CustomSet<T> {
        let mut difference_content = self.content.clone();
        difference_content.retain(|x| other_set.contains(&x));
        CustomSet {
            content: difference_content
        }
    }

    pub fn union(&self, other_set: &CustomSet<T>) -> CustomSet<T> {
        let mut union_set = CustomSet {
            content: self.content.clone()
        };
        for x in &other_set.content {
            union_set.add(*x);
        };
        union_set
    }

}
