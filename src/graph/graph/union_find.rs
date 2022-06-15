pub struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
        }
    }

    //merge the nodes a and b,
    pub fn merge(&mut self, a: usize, b: usize) -> () {
        self.parents[a] = self.find(b);
    }

    pub fn find(&mut self, mut a: usize) -> usize {
        while self.parents[a] != a {
            let a_copy = self.parents[a].clone();
            self.parents[a] = self.parents[self.parents[a]];
            a = a_copy;
        }
        a
    }

    pub fn are_adjacent(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

#[test]
fn test_union_find() {
    let mut union_find = UnionFind::new(6);
    union_find.merge(0, 1);
    union_find.merge(1, 2);
    union_find.merge(2, 3);
    union_find.merge(3, 4);

    assert_eq!(4, union_find.find(0));
    assert_eq!(4, union_find.find(1));
    assert_eq!(4, union_find.find(3));
    println!("{:?}", union_find.parents);
}
