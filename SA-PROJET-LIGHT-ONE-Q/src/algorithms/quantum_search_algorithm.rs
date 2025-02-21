/*
The goal is to simulate quantum-like behavior on a classical computer to search unsorted datasets more efficiently.
Specifically, we’ll take inspiration from Grover’s algorithm, which provides a quadratic speedup for searching unsorted databases in quantum computing.
*/
use rand::Rng;
use std::collections::HashMap;

pub fn quantum_search(dataset: &Vec<i32>, target: i32, iterations: u32) -> Option<i32> {
    println!("Running QSA v1");
    let mut rng = rand::thread_rng(); // Random number generator
    // Step 0: Initialize probabilities
    let mut probabilities: HashMap<usize, f64> = (0..dataset.len()) 
        .map(|i| (i, 1.0 / dataset.len() as f64)) 
        .collect();
    //println!("Probabilities 1: {:?}", probabilities[&0]);

    for _ in 0..iterations {
        // Step 1: Sample an element based on probabilities
        let mut cumulative_prob = 0.0;
        let random_value: f64 = rng.gen();
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
        }
    }
    //println!("Probabilities 2: {:?}", probabilities[&0]);
    // Step 4: Find the element with the highest probability
    let result = probabilities
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(&index, _)| dataset[index]);

    result
}
