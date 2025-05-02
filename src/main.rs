use degree::sort_by_count;
use plotting::*;
use regression::*;
use transform::log_transform;

mod degree; // Module for computing the in and out degree distributions, with a sort counting function to sort distributions by count descending
mod graph; // Module for parsing the file content / data into a graph
mod plotting; // Module for creating a log-log plot using points and linear regression metrics (slope + intercept)
mod regression; // Module for performing a linear regression and return slope + intercept, with additional r-squared value function.
mod transform; // Module for taking points and log transforming them.

fn main() {
    // Main function
    //
    // Usecase:
    // • Return the # of nodes in the graph using parse graph function in graph module.
    // • Return top 5 in-degree and out-degree counts in the graph by highest counts using compute degree distribution function in degree module.
    // • Return distribution metrics (in and out degree) by calculating log degrees, log counts, slope, intercept, power-law estimates, and r-squared values -
    // using the log transform function in transform module along with linear regression and r-squared functions in regression module.
    // • Finally, create in and out-degree fit plots using the log log plot function in the plotting module.

    let graph = graph::parse_graph("web-Stanford.txt");
    println!("{} nodes in graph.", graph.len());

    let degree_dists = degree::compute_degree_distribution(&graph);

    println!("\nTop in-degree counts:");
    let sorted_indeg = sort_by_count(&degree_dists.in_degrees);
    for (deg, count) in sorted_indeg.iter().take(5) {
        println!("Degree {}: {} nodes", deg, count);
    }

    println!("\nTop out-degree counts:");
    let sorted_outdeg = sort_by_count(&degree_dists.out_degrees);
    for (deg, count) in sorted_outdeg.iter().take(5) {
        println!("Degree {}: {} nodes", deg, count);
    }

    let (log_degrees_in, log_counts_in) = log_transform(&degree_dists.in_degrees);
    let (log_degrees_out, log_counts_out) = log_transform(&degree_dists.out_degrees);

    let (slope_in, intercept_in) = lin_regression(&log_degrees_in, &log_counts_in).unwrap();
    let in_deg_rsq = r_squared(&log_degrees_in, &log_counts_in, slope_in, intercept_in);

    let (slope_out, intercept_out) = lin_regression(&log_degrees_out, &log_counts_out).unwrap();
    let out_deg_rsq = r_squared(&log_degrees_out, &log_counts_out, slope_out, intercept_out);

    println!("\nIn-degree distribution Metrics:");
    println!("Slope: {:.4}, Intercept: {:.4}", slope_in, intercept_in);
    println!("Power-law exponent estimate: {:.4}", -slope_in);
    println!("R^2 (r-squared) value: {:.4}", in_deg_rsq.unwrap());

    println!("\nOut-degree distribution Metrics:");
    println!("Slope: {:.4}, Intercept: {:.4}", slope_out, intercept_out);
    println!("Power-law exponent estimate: {:.4}", -slope_out);
    println!("R^2 (r-squared) value: {:.4}", out_deg_rsq.unwrap());
    println!("\n");

    let _ = log_log_plot(
        "in_degree_fit.png",
        &log_degrees_in,
        &log_counts_in,
        slope_in,
        intercept_in,
    );

    let _ = log_log_plot(
        "out_degree_fit.png",
        &log_degrees_out,
        &log_counts_out,
        slope_out,
        intercept_out,
    );
}

#[cfg(test)]
mod test {
    use crate::{
        degree::compute_degree_distribution,
        regression::{lin_regression, r_squared},
        transform::log_transform,
    };
    use std::collections::HashMap;
    #[test]
    fn test_comp_degree_distribution() {
        let graph = vec![vec![1, 2], vec![1, 3], vec![1]];
        let degree_dists = compute_degree_distribution(&graph);
        assert_eq!(degree_dists.in_degrees.get(&3), Some(&1));
        assert_eq!(degree_dists.out_degrees.get(&2), Some(&2));
    }

    #[test]
    fn test_log_transform() {
        let mut deg_dist = HashMap::new();
        deg_dist.insert(0, 5);
        deg_dist.insert(2, 0);
        deg_dist.insert(4, 1);

        let (log_deg, log_cnt) = log_transform(&deg_dist);
        assert_eq!(log_deg.len(), 1);
        assert_eq!(log_cnt.len(), 1);

        assert!(((log_deg[0]) - 4f64.ln()).abs() < 1e-15);
        assert!(((log_cnt[0]) - 1f64.ln()).abs() < 1e-15);
    }

    #[test]
    fn test_lin_regression() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![4.0, 7.0, 10.0, 13.0, 16.0];

        let (slope, intercept) = lin_regression(&x, &y).unwrap();
        let r2 = r_squared(&x, &y, slope, intercept).unwrap();

        assert!((slope - 3.0).abs() < 1e-15);
        assert!((intercept - 1.0).abs() < 1e-15);
        assert!((r2 - 1.0).abs() < 1e-15);
    }
}
