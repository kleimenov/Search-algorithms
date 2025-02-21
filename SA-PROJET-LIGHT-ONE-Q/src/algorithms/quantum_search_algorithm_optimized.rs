// QSA_alpha v2 optimized version of the quantum search algorithm (Normalize probabilities to ensure they sum to 1.0 after amplifying the target probability)
use rand::Rng;
use std::collections::HashMap;

pub fn QSA_alpha(dataset: &Vec<i32>, target: i32, iterations: u32) -> Option<i32> {
    println!("Running QSA_alpha v2");
    let mut rng = rand::thread_rng();
    // Step 0: Initialize probabilities
    let mut probabilities: HashMap<usize, f64> = (0..dataset.len())
        .map(|i| (i, 1.0 / dataset.len() as f64))
        .collect();

    // Check if target exists in the dataset
    if !dataset.contains(&target) {
        return None;
    }

    for _ in 0..iterations {
        // Step 1: Sample an element based on probabilities
        let random_value: f64 = rng.gen();
        let mut cumulative_prob = 0.0;
        let mut selected_index = 0;

        for (index, &prob) in &probabilities {
            cumulative_prob += prob;
            if random_value <= cumulative_prob {
                selected_index = *index;
                break;
            }
        }

        // Step 2: Check if the selected element is the target
        if dataset[selected_index] == target {
            // Step 3: Amplify the probability of the target
            for (index, prob) in &mut probabilities {
                if *index == selected_index {
                    *prob *= 2.0; // Amplify target probability
                } else {
                    *prob *= 0.5; // Reduce non-target probabilities
                }
            }

            // Normalize probabilities to ensure they sum to 1.0
            let total_prob: f64 = probabilities.values().sum();
            for prob in probabilities.values_mut() {
                *prob /= total_prob;
            }
        }
    }

    // Step 4: Find the element with the highest probability
    let result = probabilities
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(&index, _)| dataset[index]);

    result
}