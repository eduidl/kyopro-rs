use std::collections::HashSet;

fn main() {
    proconio::input! {
        n: usize,
        st: [(String, i32); n],
    }

    let mut max = -1;
    let mut max_i = 0;
    let mut history = HashSet::<String>::new();
    for (i, (s, t)) in st.iter().enumerate() {
        if !history.contains(s) {
            history.insert(s.clone());

            if *t > max {
                max = *t;
                max_i = i + 1;
            }
        }
    }

    println!("{}", max_i);
}
