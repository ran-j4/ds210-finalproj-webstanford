// Graph Module -- Reads a graph from a text file and converts it into an adjacency list
use std::fs;

pub fn parse_graph(file_path: &str) -> Vec<Vec<usize>> {
    // Parse Graph Function:
    // Parses a graph file and returns an adjacency list
    //
    // Inputs:
    // • File Path - Path to the graph data file in the format of "FromNode" "ToNode" line-by-line.
    //
    // Outputs:
    // • A vector of vectors -- each index represents a node and contains a list of neighbors / outbound edges.

    let file = fs::read_to_string(file_path).expect("Unable to read file");
    let mut graph_edges = Vec::new();
    let mut max_vertex = 0;

    // For loop to iterate over lines in data file; cleaned up and stored before being pushed into edges vector.
    for line in file.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // skips blank lines and comment lines -- only nodes format to be collected
        }
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let from: usize = parts[0].parse().expect("Invalid vertex");
        let to: usize = parts[1].parse().expect("Invalid vertex");

        if from > max_vertex {
            max_vertex = from;
        }

        if to > max_vertex {
            max_vertex = to;
        }

        graph_edges.push((from, to));
    }

    let mut graph = vec![Vec::new(); max_vertex + 1]; // use of max_vertex to allocate graph size (adjacency list representation)
    for (from, to) in graph_edges {
        graph[from].push(to);
    }

    graph
}
