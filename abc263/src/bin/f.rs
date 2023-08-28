fn main() {
    proconio::input! {
        n: usize,
        c: [[u64; n]; 2u32.pow(n as u32) as usize],
    }

    let mut dp = vec![vec![0; c.len()]; n];

    for i in 1..n {
        for j in 0..2i32.pow((n - i - 1) as u32) {
            let size = 2i32.pow(i as u32) as usize;
            let start = j as usize * 2 * size;

            let l_max = dp[i - 1]
                .iter()
                .enumerate()
                .skip(start)
                .take(size)
                .map(|(idx, v)| v + c[idx][i - 1])
                .max()
                .unwrap();

            let r_max = dp[i - 1]
                .iter()
                .enumerate()
                .skip(start + size)
                .take(size)
                .map(|(idx, v)| v + c[idx][i - 1])
                .max()
                .unwrap();

            for k in 0..size {
                dp[i][k + start] = dp[i - 1][k + start] + r_max;
                dp[i][k + start + size] = dp[i - 1][k + start + size] + l_max;
            }
        }
    }

    let max = dp[n - 1]
        .iter()
        .enumerate()
        .map(|(idx, v)| v + c[idx][n - 1])
        .max()
        .unwrap();

    println!("{}", max);
}
