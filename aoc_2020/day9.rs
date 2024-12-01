use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parser() -> Vec<usize> {
    let input = include_str!("../inputs/2020/day9.input");
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

pub fn part1() -> usize {
    'outer: for [ls @ .., end] in parser().array_windows::<26>() {
        for x in ls {
            for y in ls {
                if *x + *y == *end {
                    continue 'outer;
                }
            }
        }
        return *end;
    }
    56
}

pub fn part2() -> usize {
    let invalid = part1();
    let inp = parser();
    (0..inp.len())
        .into_par_iter()
        .filter_map(|x| {
            for y in x + 1..inp.len() {
                if inp[x..y].iter().copied().sum::<usize>() == invalid {
                    let mut c = inp[x..y].to_vec();
                    c.sort();
                    return Some(c[0] + c[c.len() - 1]);
                }
            }
            None
        })
        .find_any(|_| true)
        .unwrap()
}
