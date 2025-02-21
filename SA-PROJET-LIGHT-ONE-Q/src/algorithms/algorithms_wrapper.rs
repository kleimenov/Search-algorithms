use crate::algorithms::quantum_search_algorithm_optimized::QSA_alpha;
use crate::algorithms::quantum_search_algorithm::quantum_search;

// function run quantum search algorithm 10 times and return accuracy of results
pub fn _wrapper(dataset: &Vec<i32>, target: i32, iterations: u32) -> f64 {
    let mut total_accuracy = 0.0;
    for (idx, _) in (0..10).enumerate() {
        let result = quantum_search(&dataset, target, iterations);
        println!("Iteration {}, Result gussed value as target: {:?}", idx + 1, result.unwrap());
        let is_match = target == result.unwrap();
        total_accuracy += is_match as i32 as f64;
    }

    // TODO: Improve accuracy calculation
    // Add some other metrics to calculate accuracy: Confidence Intervals / Bayesian Estimation / Precision, Recall, and F1-Score /Area Under the ROC Curve (AUC-ROC)



    total_accuracy / 10.0
}