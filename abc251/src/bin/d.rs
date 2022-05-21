fn main() {
    proconio::input! {
        _w: i32,
    }

    let mut first = true;
    println!("297");
    for a in &[1, 100, 10000] {
        for i in 1..100 {
            if first {
                first = false;
            } else {
                print!(" ");
            }
            print!("{}", i * a);
        }
    }

    println!();
}
