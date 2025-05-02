// Transform Module -- logarithm transforms degrees and counts of a degree distribution
use std::collections::HashMap;

pub fn log_transform(degree_dist: &HashMap<usize, usize>) -> (Vec<f64>, Vec<f64>) {
    // Log Transform Function:
    // Takes a degree distribution of degrees and counts (as a HashMap) and returns a tuple of f64 vectors
    // containing natural log transforms of degrees and counts stored in each vector respectively
    //
    // Inputs:
    // • Degree Distribution -- Referenced HashMap containing degrees and their respective counts
    //
    // Outputs:
    // • Tuple consisting of 2 f64 type vectors containing the natural log values of the degrees and their counts.
    //
    // Key Logic:
    // • Use for loop and iterate over the degree distribution to get degrees and counts and use them in calculations.

    let mut log_degrees = Vec::new();
    let mut log_counts = Vec::new();

    for (&deg, &ct) in degree_dist.iter() {
        if deg > 0 && ct > 0 {
            log_degrees.push((deg as f64).ln());
            log_counts.push((ct as f64).ln());
        } // checks if degree and count are greater than 0 before taking natural log and pushing to respective vectors.
    }
    (log_degrees, log_counts)
}
