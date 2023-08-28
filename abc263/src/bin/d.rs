fn main() {
    proconio::input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }

    let mut lmin = vec![0];
    for i in 0..n {
        lmin.push((lmin[i] + a[i]).min(l * ((i + 1) as i64)));
    }

    let mut rmin = vec![0];
    for i in 0..n {
        rmin.push((rmin[i] + a[n - 1 - i]).min(r * ((i + 1) as i64)));
    }

    let min = lmin
        .iter()
        .zip(rmin.iter().rev())
        .map(|(l, r)| *l + *r)
        .min()
        .unwrap();

    println!("{}", min);
}
