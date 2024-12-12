use rustc_hash::FxHashMap;
use utils::MatrixGet;
matrixy::matrixy!("../inputs/2024/day12.input");

pub fn part1() -> usize {
    let mut map = MAP
        .iter()
        .map(|s| {
            s.iter()
                .filter(|c| **c != b'\n')
                .copied()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            if c == b'.' {
                continue;
            }
            let mut stack = vec![(y, x)];
            let mut to_remove = Vec::new();
            while let Some((y, x)) = stack.pop() {
                to_remove.push((y, x));
                if let Some(v) = map.mget(y, x, -1, 0) {
                    if *v == c && !to_remove.contains(&(y - 1, x)) {
                        stack.push((y - 1, x))
                    }
                };
                if let Some(v) = map.mget(y, x, 1, 0) {
                    if *v == c && !to_remove.contains(&(y + 1, x)) {
                        stack.push((y + 1, x))
                    }
                };
                if let Some(v) = map.mget(y, x, 0, -1) {
                    if *v == c && !to_remove.contains(&(y, x - 1)) {
                        stack.push((y, x - 1))
                    }
                };
                if let Some(v) = map.mget(y, x, 0, 1) {
                    if *v == c && !to_remove.contains(&(y, x + 1)) {
                        stack.push((y, x + 1))
                    }
                };
            }

            to_remove.sort();
            to_remove.dedup();
            let area = to_remove.len();
            let mut perimiter = FxHashMap::default();
            for (y, x) in to_remove {
                let y = y + 1;
                let x = x + 1;
                perimiter.insert((y, x), 60);
                *perimiter.entry((y - 1, x)).or_insert(0) += 1;
                *perimiter.entry((y + 1, x)).or_insert(0) += 1;
                *perimiter.entry((y, x - 1)).or_insert(0) += 1;
                *perimiter.entry((y, x + 1)).or_insert(0) += 1;
                map[y - 1][x - 1] = b'.';
            }
            let perimiter = perimiter.into_values().filter(|c| *c <= 4).sum::<usize>();
            sum += area * perimiter;
            // println!("{}: {area} | {perimiter}", c as char)
        }
    }

    sum
}
pub fn part2() -> i64 {
    0
}
