fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n - 1],
    }

    let mut idx = n;
    let mut cnt = 0;
    while idx != 1 {
        idx = p[idx - 2];
        cnt += 1;
    }

    println!("{}", cnt);
}
