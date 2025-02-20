use rand::Rng;
use std::time::Instant;
mod algorithms;

fn main() {
    println!("Search Algorithms");
    println!("EXPERIMENT 1 quantum inspired classic search: --------------------------- START");

    let mut rng = rand::thread_rng();
    //let mut dataset: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
    let mut dataset: Vec<i32> = (0..100000).map(|_| rng.gen_range(0..100)).collect(); // 100 thousand elements ~ 0.2 seconds
    //let mut dataset: Vec<i32> = (0..10000000).map(|_| rng.gen_range(0..100)).collect(); // 10 million elements ~ 20 seconds
    //let mut dataset: Vec<i32> = (0..100000000).map(|_| rng.gen_range(0..100)).collect(); // 100 million elements ~ 200 seconds
    //let mut dataset: Vec<i32> = (0..1000000000).map(|_| rng.gen_range(0..100)).collect(); // 1 billion elements ~ have not tested yet but should be around 2000 seconds

    dataset.push(20); // Add the target to the dataset
    let target: i32 = 20;
    let iterations: usize = 200;
    
    // TEST FOR SINGLE RUN
    // // Benchmark quantum search
    // let start = Instant::now();
    // let result = run_quantum_search(&dataset, target, iterations);
    // let duration = start.elapsed();

    // let duration_in_seconds: f64 = duration.as_secs_f64();
    // let is_match: bool = target == result.unwrap();

    // // RESULTS
    // println!("Quantum search completed.");
    // println!("Quantum Search took: {duration_in_seconds:.6} seconds");
    // println!("Target was found: {is_match}");
    // println!("Target value was: {}, Algorithm found: {:?}", target, result.unwrap());
    // println!("Algorithm found target in: {iterations} iterations");
    // println!("There are {} elements in dataset", &dataset.len() -1);
    // println!("--------------------------------------------------------------------------------");

    // TEST FOR MULTIPLE RUNS
    // Benchmark quantum search
    let start = Instant::now();
    let result = multiple_run_quantum_search(&dataset, target, iterations);
    let duration = start.elapsed();

    let duration_in_seconds: f64 = duration.as_secs_f64();

    // RESULTS
    println!("Quantum search completed.");
    println!("Quantum Search took: {duration_in_seconds:.6} seconds");
    println!("There are {} elements in dataset", &dataset.len() -1);
    println!("Mean Accuracy after 10 iteration (initial implementatin WIP) is : {}", result);
    println!("--------------------------------------------------------------------------------");
}

// TODO: Move utils function to a separate file
fn run_hash_table_lookup(dataset: &Vec<i32>, target: i32) -> bool {
    let result = algorithms::classic_search::hash_table_lookup(&dataset, target);
    result
}

fn run_linear_search(dataset: &Vec<i32>, target: i32) -> Option<usize> {
    let result = algorithms::classic_search::linear_search(&dataset, target);
    result
}

fn run_quantum_search(dataset: &Vec<i32>, target: i32, iterations: usize) -> Option<i32> {
    let result = algorithms::quantum_search_algorithm::quantum_search(&dataset, target, iterations);
    result
}

fn multiple_run_quantum_search(dataset: &Vec<i32>, target: i32, iterations: usize) -> f64 {
    let result = algorithms::quantum_search_algorithm::quantum_search_algorithm_wrapper(&dataset, target, iterations);
    result
}