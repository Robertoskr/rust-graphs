
pub struct Node {
    to: usize, 
    weight: f64,
}

pub struct Graph {
    edges: Vec<Vec<Node>>
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: Vec::new()
        }
    }

    pub fn with_capacity(cap: &usize) -> Self {
        Self {
            edges: vec![vec![];*cap],
        }
    }

    //compute the distance between the two nodes (from and to) 
    //and return the distance, and the distances vector
    //for using the distance vector you first need to compute the paths along them 
    pub fn bidirectional_search(&self, from: usize, to: usize) -> (i64, Vec<i64>) {

    }
}