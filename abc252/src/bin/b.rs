use std::collections::HashSet;

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [i32; n],
        b: [Usize1; k],
    }

    let mut max = 0;
    let mut max_set = HashSet::new();
    for (i, &aa) in a.iter().enumerate() {
        if aa > max {
            max = aa;
            max_set.clear();
            max_set.insert(i);
        } else if aa == max {
            max_set.insert(i);
        }
    }

    let mut ans = false;
    for bb in b {
        if max_set.contains(&bb) {
            ans = true;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
