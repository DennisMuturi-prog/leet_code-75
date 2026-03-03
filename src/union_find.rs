pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn make_set(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        Self { parent, rank }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[root_y] = root_x;
        } else if self.rank[y] < self.rank[x] {
            self.parent[root_x] = root_y;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
    }
}
