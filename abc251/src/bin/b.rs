fn main() {
    proconio::input! {
        n: usize,
        w: usize,
        mut a: [i32; n],
    }
    a.push(0);
    a.push(0);

    let mut vec = vec![false; w + 1];

    for i in 0..n {
        for j in i + 1..n + 1 {
            for k in j + 1..n + 2 {
                let ww = (a[i] + a[j] + a[k]) as usize;
                if ww <= w {
                    vec[ww] = true;
                }
            }
        }
    }

    println!("{}", vec.iter().filter(|v| **v).count());
}
