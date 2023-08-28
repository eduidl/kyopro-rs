use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    proconio::input! {
        n: usize,
        l: u64,
        a: [u64; n],
    }

    let mut pr_queue = BinaryHeap::new();

    for &aa in a.iter() {
        pr_queue.push(Reverse(aa));
    }
    let remain = l - a.iter().sum::<u64>();
    if remain > 0 {
        pr_queue.push(Reverse(remain));
    }

    let mut ans = 0u64;
    while pr_queue.len() > 1 {
        let a = pr_queue.pop().unwrap();
        let b = pr_queue.pop().unwrap();
        ans += a.0 + b.0;
        pr_queue.push(Reverse(a.0 + b.0));
    }

    println!("{}", ans);
}
