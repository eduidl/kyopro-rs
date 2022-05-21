use std::{cmp, i64};

fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = vec![i64::MAX; n + 1];
    dp[0] = 0;
    let mut dp_n = vec![i64::MAX; n + 1];
    dp_n[0] = 0;
    dp_n[1] = 0;

    for i in 0..n - 1 {
        if dp[i] != i64::MAX {
            dp[i + 1] = cmp::min(dp[i + 1], dp[i] + a[i]);
            dp[i + 2] = cmp::min(dp[i + 2], dp[i] + a[i]);
        }
        if dp_n[i] != i64::MAX {
            dp_n[i + 1] = cmp::min(dp_n[i + 1], dp_n[i] + a[i]);
            dp_n[i + 2] = cmp::min(dp_n[i + 2], dp_n[i] + a[i]);
        }
    }

    println!("{}", cmp::min(dp[n], dp_n[n - 1] + a[n - 1]));
}
