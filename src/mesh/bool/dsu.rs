use crate::mesh::{MeshError, MeshResult};

struct UnionFindSet {
    ids: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFindSet {
    pub fn new(n: usize) -> Self {
        let mut ids = Vec::with_capacity(n);
        let mut rank = Vec::with_capacity(n);
        for i in 0..n {
            ids.push(i);
            rank.push(0);
        }
        Self { ids, rank }
    }

    pub fn find(&mut self, mut p: usize) -> MeshResult<usize> {
        if p >= self.ids.len() {
            Err(MeshError::InvalidIndex(format!("index {} out of bounds", p)))
        } else {
            while p != self.ids[p] {
                self.ids[p] = self.ids[self.ids[p]];
                p = self.ids[p];
            }
            Ok(p)
        }
    }

    pub fn union(&mut self, p: usize, q: usize) -> MeshResult<()> {
        let root_p = self.find(p)?;
        let root_q = self.find(q)?;
        if root_p != root_q {
            if self.rank[root_p] < self.rank[root_q] {
                self.ids[root_p] = root_q;
            } else if self.rank[root_p] > self.rank[root_q] {
                self.ids[root_q] = root_p;
            } else {
                self.ids[root_p] = root_q;
                self.rank[root_q] += 1;
            }
        }

        Ok(())
    }
}
