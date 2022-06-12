mod graph;

use graph::graph::Graph;
use std::env;

fn main() {
    //TODO: support args for doing different operations
    let args: Vec<String> = env::args().collect();
    let mut graph = Graph::from_prompt();

    let mut visit = move |value: usize| {
        println!("{}", value);
    };

    println!("{:?}", graph.bfs(0, visit, visit));
}
