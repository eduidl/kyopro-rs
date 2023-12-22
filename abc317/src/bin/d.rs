fn main() {
    proconio::input! {
        n: usize,
        xyz: [(u32, u32, usize); n],
    }

    let z_sum = xyz.iter().map(|(_, _, z)| z).sum::<usize>();

    const MAX: u64 = 1e18 as u64;

    let mut dp = vec![MAX; z_sum + 1];
    dp[0] = 0;

    for (x, y, z) in xyz.into_iter() {
        let cost = ((x + y) / 2 + 1).saturating_sub(x);

        for i in (z..(z_sum + 1)).rev() {
            dp[i] = dp[i].min(dp[i - z] + cost as u64);
        }
    }

    let ans = dp.iter().skip(z_sum / 2 + 1).min().unwrap();

    println!("{}", ans);
}
