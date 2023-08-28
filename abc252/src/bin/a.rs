fn main() {
    proconio::input! { n: usize }

    let az = "abcdefghijklmnopqrstuvwxyz";
    println!("{}", az.chars().nth(n - 97).unwrap());
}
