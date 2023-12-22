fn main() {
    proconio::input! {
        n: usize,
        h: u32,
        x: u32,
        p: [u32; n],
    }

    let (i, _) = p
        .iter()
        .enumerate()
        .filter(|(_, &pp)| pp + h >= x)
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    println!("{}", i + 1);
}
