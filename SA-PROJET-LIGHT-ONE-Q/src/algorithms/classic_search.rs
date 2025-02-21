use std::collections::HashSet;

// Single Search (One-Time Lookup) linear time
pub fn linear_search(dataset: &Vec<i32>, target: i32) -> Option<usize> {
    for (index, &value) in dataset.iter().enumerate() {
        if value == target {
            return Some(index); // Return the index if the target is found
        }
    }
    None // Return None if the target is not found
}

// hash table lookup
pub fn hash_table_lookup(dataset: &Vec<i32>, target: i32) -> bool {
    // Create a HashSet from the dataset
    let hash_set: HashSet<i32> = dataset.iter().cloned().collect();

    // Check if the target exists in the HashSet
    hash_set.contains(&target)
}