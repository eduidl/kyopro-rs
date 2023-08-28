use std::usize;

fn main() {
    proconio::input! {
        n: usize,
        s: [String; n],
    }

    let mut count = vec![[0; 10]; 10];
    for ss in s {
        for (i, c) in ss.chars().enumerate() {
            count[c.to_digit(10).unwrap() as usize][i] += 1;
        }
    }

    let mut ans = usize::MAX;
    for i in 0..10 {
        let mut cur = 0;
        for j in 0..10 {
            if count[i][j] > 0 {
                cur = cur.max((count[i][j] - 1) * 10 + j);
            }
        }
        ans = ans.min(cur);
    }

    println!("{}", ans);
}
