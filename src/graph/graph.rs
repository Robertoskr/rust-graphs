#![allow(dead_code)]
#![allow(unused_variables)]
mod union_find;
mod utils;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use union_find::*;
use utils::*;

#[derive(Clone, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    from: usize,
    to: usize,
    weight: i64,
}

impl Node {
    fn new(from: usize, to: usize, weight: i64) -> Self {
        Self { from, to, weight }
    }
}

pub struct Graph {
    pub edges: Vec<Vec<Node>>,
    pub points: Vec<Point>,
    total_edges: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            points: Vec::new(),
            total_edges: 0,
        }
    }

    pub fn with_capacity(cap: &usize) -> Self {
        Self {
            edges: vec![Vec::new(); *cap],
            points: Vec::with_capacity(*cap),
            total_edges: 0,
        }
    }
    //sets the given points in to the graph
    pub fn set_points(&mut self, mut points: Vec<(usize, Point)>) {
        points.sort_by(|x, y| x.0.cmp(&y.0));
        for (i, point) in points.iter().enumerate() {
            assert_eq!(i, point.0);
            self.points.push(point.1.clone());
        }
    }

    //creates a new graph from command prompt input
    //example input:
    //1
    //0 0 1
    pub fn from_prompt() -> Self {
        let (m, n) = utils::read_two_numbers();
        let mut graph = Self::with_capacity(&(m as usize));
        for _ in 0..n {
            let (from, to, weight) = utils::read_three_numbers();
            graph.add_weighted_edge(from as usize, to as usize, weight);
        }
        graph
    }

    //TODO: support creating a graph from a file
    fn from_file(filename: String) -> Self {
        Self::new()
    }

    //return the total of edges in the graph
    pub fn total_edges(&self) -> usize {
        self.total_edges
    }

    pub fn add_weighted_edge(&mut self, from: usize, to: usize, weight: i64) -> () {
        self.edges[from].push(Node::new(from, to, weight));
    }

    //compute the distance between the two nodes (from and to)
    //and return the distance, and the distances vector
    //for using the distance vector you first need to compute the paths along them
    //this algorithm works always with unweighted graphs
    //but with weighted graphs, can fail in some edge cases.
    pub fn bidirectional_search(&self, from: usize, to: usize) -> Option<(i64, Vec<i64>)> {
        let mut heap_a = BinaryHeap::new();
        let mut heap_b = BinaryHeap::new();
        let mut distances: Vec<i64> = vec![i64::MAX; self.edges.len()];
        let mut group_a: HashSet<usize> = HashSet::new();
        let mut group_b: HashSet<usize> = HashSet::new();
        let mut middle_node: Option<Node> = None;

        //prepare for performing the algorithm
        heap_a.push(Reverse(from));
        heap_b.push(Reverse(to));
        distances[from] = 0;
        distances[to] = 0;
        let mut direction = false; //depending on this we execute from one side or the other side
        let mut reference: &mut BinaryHeap<Reverse<usize>>;
        while !heap_a.is_empty() || !heap_b.is_empty() {
            match direction {
                true => {
                    reference = &mut heap_a;
                }
                false => {
                    reference = &mut heap_b;
                }
            }

            if reference.is_empty() {
                direction = !direction;
                continue;
            }
            let actual: usize = reference.pop().unwrap().0;

            for edge in &self.edges[actual] {
                let dist = distances[actual] + edge.weight;
                if dist < distances[edge.to]
                    || (distances[edge.to] == 0 && (edge.to == from || edge.to == to))
                {
                    distances[edge.to] = dist;
                    reference.push(Reverse(edge.to));
                }
                if direction {
                    group_a.insert(edge.to);
                    if group_b.contains(&edge.to) {
                        middle_node = Some(edge.clone());
                        break;
                    }
                } else {
                    group_b.insert(edge.to);
                    if group_a.contains(&edge.to) {
                        middle_node = Some(edge.clone());
                        break;
                    }
                }
            }
            direction = !direction;
        }

        //if no middle node, the two points are unreachable
        match middle_node {
            Some(node) => {
                //god path has been found by the algorithm
                return Some((
                    distances[node.from] + distances[node.to] + node.weight,
                    distances,
                ));
            }
            None => {
                //No middle node can mean that the node has no edges to other parts (is a synk) or that this node is unreachable
                if distances[from] + distances[to] == i64::max(distances[from], distances[to]) {
                    return Some((i64::max(distances[from], distances[to]), distances));
                } else {
                    return None;
                }
            }
        }
    }

    //dijsktra algorithm, for computing the best path from one node to all nodes in the graph
    //works in (O*V) worst case
    //the graph is supposed to not have negative cycles, if you have a graph with negative cycles,
    //use bellman ford algorithm for that, and detect the negative cycles.
    pub fn dijsktra(&self, start: usize) -> Vec<i64> {
        let mut distances = vec![i64::MAX; self.edges.len()];
        let mut heap: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
        heap.push((Reverse(0), start));

        while !heap.is_empty() {
            let actual = heap.pop().unwrap().1;
            for edge in &self.edges[actual] {
                let dist = distances[edge.from] + edge.weight;
                if dist < distances[edge.to] {
                    distances[edge.to] = dist;
                    heap.push((Reverse(distances[edge.to]), edge.to));
                }
            }
        }

        distances
    }

    //bfs algorithm
    //returns the distance from start node to all nodes
    //distance is measured on n of edges between two vertices
    pub fn bfs(
        &self,
        start: usize,
        previsit: impl Fn(usize),
        postvisit: impl Fn(usize),
    ) -> Vec<i64> {
        let mut distances = vec![i64::MAX; self.edges.len()];
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; self.edges.len()];
        queue.push_front(start);
        distances[start] = 0;
        while !queue.is_empty() {
            if visited[*queue.back().unwrap()] {
                postvisit(queue.pop_back().unwrap());
                continue;
            }
            let actual = queue.back().unwrap();
            visited[*actual] = true;
            previsit(*actual);
            for edge in &self.edges[*actual] {
                if !visited[edge.to] {
                    distances[edge.to] = distances[edge.from] + 1;
                    queue.push_front(edge.to);
                }
            }
        }
        distances
    }

    fn a_search_heuristic(&self, a: &Point, b: &Point) -> i64 {
        i64::abs(a.x - b.x) + i64::abs(a.y - b.y)
    }

    pub fn a_search(&self, start: usize, target: usize) -> i64 {
        let mut distances: Vec<i64> = vec![i64::MAX; self.edges.len()];
        let mut parents: Vec<i64> = vec![-1; self.edges.len()];
        let mut heap: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
        distances[start] = 0;
        heap.push((Reverse(0), start));

        while !heap.is_empty() {
            let actual = heap.pop().unwrap().1;
            if actual == target {
                break;
            }
            for edge in &self.edges[actual] {
                let dist = distances[actual] + edge.weight;
                if parents[edge.to] == -1 || distances[edge.to] > dist {
                    parents[edge.to] = actual as i64;
                    distances[edge.to] = dist;
                    //heuristic of a*search
                    let priority =
                        self.a_search_heuristic(&self.points[edge.to], &self.points[target]);
                    heap.push((Reverse(priority as i64), edge.to));
                }
            }
        }
        distances[target]
    }

    //compute the minimum spanning tree using prim's algorithm
    pub fn minimum_spanning_tree(&self) -> Vec<Node> {
        let mut edges: Vec<Node> = Vec::with_capacity(self.total_edges());
        let mut union_find = UnionFind::new(self.edges.len());
        let mut result_edges: Vec<Node> = Vec::with_capacity(self.edges.len() - 1);
        //create all the edges
        for _edges in &self.edges {
            for _edge in _edges {
                edges.push(_edge.clone());
            }
        }
        //sort the edges
        edges.sort_by(|a, b| a.weight.cmp(&b.weight));
        //start picking edges
        let mut i = 0;
        while result_edges.len() < self.edges.len() {
            let actual_edge = edges[i].clone();
            if !union_find.are_adjacent(actual_edge.from, actual_edge.to) {
                union_find.merge(actual_edge.from, actual_edge.to);
                result_edges.push(actual_edge);
            }
        }
        result_edges
    }

    //perform the topological sort of a graph
    fn topological_sort(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        //perform dfs and keep adding nodes to the dfs
        let mut stack: Vec<usize> = Vec::new();
        let mut visited: Vec<bool> = vec![false; self.edges.len()];
        let mut topological_sort: Vec<usize> = Vec::with_capacity(self.edges.len());

        //perform dijstra algorithm, for computing the topological sort
        for i in 0..self.edges.len() {
            stack.push(i);
            while !stack.is_empty() {
                let actual = stack[self.edges.len() - 1];
                if visited[actual] {
                    stack.pop();
                    topological_sort.push(actual);
                }
                visited[actual] = true;
                for edge in &self.edges[actual] {
                    if !visited[edge.to] {
                        stack.push(edge.to);
                    }
                }
            }
        }
        //reverse the order of the vector for being the correct
        topological_sort.reverse();
        topological_sort
    }

    //TODO: add network flow algorithms
    //TODO: add connected components algorithms
    fn number_of_components(&self) -> usize {
        let mut visited: Vec<bool> = vec![false; self.edges.len()];
        let mut stack: Vec<usize> = Vec::new();
        let mut result: usize = 0;
        //perform dijstra algorithm, for computing the topological sort
        for i in 0..self.edges.len() {
            if !visited[i] {
                stack.push(i);
            } else {
                result += 1;
            }
            while !stack.is_empty() {
                let actual = stack[self.edges.len() - 1];
                if visited[actual] {
                    stack.pop();
                }
                visited[actual] = true;
                for edge in &self.edges[actual] {
                    if !visited[edge.to] {
                        stack.push(edge.to);
                    }
                }
            }
        }
        result
    }
}
