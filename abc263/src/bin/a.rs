use std::collections::HashMap;

fn main() {
    proconio::input! {
        a: [u8; 5],
    }

    let mut map = HashMap::new();
    for aa in a {
        let counter = map.entry(aa).or_insert(0);
        *counter += 1;
    }

    let mut has_two = false;
    let mut has_three = false;
    for v in map.values() {
        match v {
            2 => has_two = true,
            3 => has_three = true,
            _ => (),
        }
    }

    println!("{}", if has_two && has_three { "Yes" } else { "No" });
}
