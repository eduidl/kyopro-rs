use proconio::marker::Usize1;

struct Global {
    n: usize,
    max: u32,
    e: Vec<Vec<u32>>,
    used: Vec<bool>,
}

impl Global {
    fn dfs(&mut self, i: usize, sum: u32) {
        self.used[i] = true;
        self.max = self.max.max(sum);

        for j in 0..self.n {
            if !self.used[j] && self.e[i][j] > 0 {
                self.dfs(j, sum + self.e[i][j]);
            }
        }
        self.used[i] = false;
    }
}

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u32); m],
    }

    let mut g = Global {
        n,
        max: 0,
        e: vec![vec![0; n]; n],
        used: vec![false; n],
    };

    for (a, b, c) in abc.into_iter() {
        g.e[a][b] = c;
        g.e[b][a] = c;
    }

    for i in 0..n {
        g.dfs(i, 0);
    }

    println!("{}", g.max);
}
