fn main() {
    proconio::input! {
        n: usize,
        mut a: [u32; n],
    }

    a.sort_unstable();
    let mut dp = [vec![1; n], vec![0; n], vec![0; n]];

    for i in 0..2 {
        let mut acc = 0;
        let mut local_acc = 0;
        for j in 0..n - 1 {
            if a[j] == a[j + 1] {
                local_acc += dp[i][j];
            } else {
                acc += dp[i][j] + local_acc;
                local_acc = 0;
            }
            dp[i + 1][j + 1] = acc;
        }
    }

    let sum: u64 = dp[2].iter().sum();
    println!("{}", sum);
}
