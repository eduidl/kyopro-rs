use std::collections::VecDeque;

use proconio::marker::Chars;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    }

    let mut penetrable = vec![vec![true; w]; h];

    let mut start = None;
    let mut goal = None;

    for (r, row) in a.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            match char {
                'S' => start = Some((r, c)),
                'G' => goal = Some((r, c)),
                '#' => penetrable[r][c] = false,
                _ => (),
            }
        }
    }

    let start = start.unwrap();
    let goal = goal.unwrap();

    let check = |r: i32, c: i32| -> bool { 0 <= r && r < h as i32 && 0 <= c && c < w as i32 };
    const D: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut view = |r: usize, c: usize, di: usize| {
        let (dr, dc) = D[di];
        let mut r = r as i32;
        let mut c = c as i32;
        let mut first = true;
        while check(r, c) && (first || a[r as usize][c as usize] == '.') {
            first = false;
            penetrable[r as usize][c as usize] = false;
            r += dr;
            c += dc;
        }
    };

    for (r, row) in a.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            match char {
                'v' => view(r, c, 0),
                '>' => view(r, c, 1),
                '^' => view(r, c, 2),
                '<' => view(r, c, 3),
                _ => (),
            }
        }
    }

    const UNKNOWN: i32 = -1;
    let mut maze = vec![vec![UNKNOWN; w]; h];
    let mut queue = VecDeque::new();

    maze[start.0][start.1] = 0;
    queue.push_back(start);
    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in D.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if !check(nr, nc)
                || !penetrable[nr as usize][nc as usize]
                || maze[nr as usize][nc as usize] != UNKNOWN
            {
                continue;
            }
            maze[nr as usize][nc as usize] = maze[r][c] + 1;
            queue.push_back((nr as usize, nc as usize));
        }
    }

    println!("{}", maze[goal.0][goal.1]);
}
