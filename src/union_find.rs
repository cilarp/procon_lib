#[allow(dead_code)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    #[allow(dead_code)]
    fn init(n: usize) -> Self {
        let mut parent: Vec<usize> = Vec::new();
        let mut rank: Vec<usize> = Vec::new();
        for i in 0..n {
            parent.push(i);
            rank.push(0);
        }

        Self { parent, rank }
    }

    #[allow(dead_code)]
    fn root(&self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.root(self.parent[x])
        }
    }

    #[allow(dead_code)]
    fn same(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}
