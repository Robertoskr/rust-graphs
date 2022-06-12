mod graph;

use graph::graph::Graph;
use std::env;

fn main() {
    //TODO: support args for doing different operations
    let args: Vec<String> = env::args().collect();
    let mut graph = Graph::from_prompt();
    println!(
        "{:?}",
        graph
            .bidirectional_search(0, graph.edges.len() - 1)
            .unwrap()
    );
}
