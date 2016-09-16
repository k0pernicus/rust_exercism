use std::collections::BTreeMap;

pub fn transform(ordered_by_string: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {

    let mut ordered_by_values: BTreeMap<String, i32> = BTreeMap::new();

    for (value, string_vectors) in ordered_by_string {
        for string in string_vectors {
            ordered_by_values.insert(string.clone().to_lowercase(), *value);
        }
    }

    ordered_by_values

}
