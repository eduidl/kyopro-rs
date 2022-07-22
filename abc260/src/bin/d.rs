use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut out = vec![-1; n];

    let mut vec = Vec::<Vec<usize>>::new();

    for (tern, pp) in p.into_iter().enumerate() {
        let i = match vec.binary_search_by_key(&pp, |v| *v.last().unwrap()) {
            Ok(_) => unreachable!(),
            Err(i) if i >= vec.len() => {
                vec.push(vec![pp]);
                i
            }
            Err(i) => {
                vec[i].push(pp);
                i
            }
        };

        if vec[i].len() == k {
            for v in vec.remove(i) {
                out[v - 1] = tern as i32 + 1;
            }
        }
    }

    for v in out {
        println!("{}", v);
    }
}
