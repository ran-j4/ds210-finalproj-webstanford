// Degree Module -- computes in and out degree distributions and provides a count sorter function
use std::collections::HashMap;
pub struct DegreeDistributions {
    pub in_degrees: HashMap<usize, usize>,
    pub out_degrees: HashMap<usize, usize>,
} // DegreeDistributions struct to create a datatype that stores both in and out degree distributions of a graph,
  // and is used for calling specific distribution to conduct log transforming, regression, and plotting operations.

pub fn compute_degree_distribution(graph: &Vec<Vec<usize>>) -> DegreeDistributions {
    // Compute Degree Distribution Function:
    // Uses a graph (adjacency list representation) to compute in and out degree distributions stored in a "DegreeDistributions" datatype
    //
    // Inputs:
    // • Graph (as adjacency list format)
    //
    // Outputs:
    // • Degree Distributions datatype consisting of two HashMaps (in degree and out degree distributions, with each HashMap containing degree and counts).
    //
    // Key Logic:
    // • Use for loop to iterate and enumerate graph to fill out-degree distribution,
    // then iterate through adjacency list / neighbors to calculate in-degrees,
    // and iterate over that to fill out in-degree distribution, and then return both distributions.

    let mut in_degrees = vec![0usize; graph.len()];
    let mut in_degree_count = HashMap::new();
    let mut out_degree_count = HashMap::new();

    for (_i, neighbors) in graph.iter().enumerate() {
        let out_degree = neighbors.len();
        *out_degree_count.entry(out_degree).or_insert(0) += 1; // add out degree (length of neighbors) to out degree distribution HashMap

        for &neighbor in neighbors {
            if neighbor < in_degrees.len() {
                in_degrees[neighbor] += 1;
            }
        } // iterate through neighbors and if neighbors < length of in degrees vector, then increment neighbor index of in degrees by 1.
    } // iterate and enumerate through graph

    for &in_degree in &in_degrees {
        *in_degree_count.entry(in_degree).or_insert(0) += 1;
    } // iterate through in degrees vector and add in degree values to in degree distribution HashMap

    DegreeDistributions {
        in_degrees: in_degree_count,
        out_degrees: out_degree_count, // return in and out degree distributions as part of DegreeDistributions
    }
}

pub fn sort_by_count(degree_dist: &HashMap<usize, usize>) -> Vec<(usize, usize)> {
    // Sort By Count Function:
    // Takes a degree distribution HashMap, sorts by counts in descending order (highest to lowest), and returns vector of tuples.
    //
    // Inputs:
    // • Degree Distribution reference (HashMap with degree and count)
    //
    // Outputs:
    // • Vector consisting of tuples of degree and count.
    //
    // Key Logic:
    // • Iterate through degree distribution and collect values into a vector,
    // then sort vector by key and reverse counts before iterating through references and collecting actual values.
    let mut sorted_vec: Vec<_> = degree_dist.iter().collect();
    sorted_vec.sort_by_key(|&(_degree, &count)| std::cmp::Reverse(count));
    sorted_vec
        .into_iter()
        .map(|(&deg, &cnt)| (deg, cnt))
        .collect()
}
