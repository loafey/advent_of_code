use utils::{bi_functors::*, load_string, FoldDefault, ParseAndCollect};

fn inputs() -> impl Iterator<Item = (usize, usize)> {
    let s = load_string("inputs/2023/day6.input");
    s.split_once('\n')
        .unwrap()
        .splet(|i| {
            i.split_whitespace()
                .skip(1)
                .parse_and_collect::<Vec<_>, usize>()
        })
        .splot(|t, d| t.into_iter().zip(d))
}

fn calc_winners(time: usize, distance: usize) -> usize {
    (0..=time)
        .filter_map(|hold_time| (((time - hold_time) * hold_time) > distance).then_some(1))
        .sum()
}

pub fn part1() -> usize {
    inputs()
        .map(|(time, distance)| calc_winners(time, distance))
        .product()
}

pub fn part2() -> usize {
    inputs()
        .fold_d(|(ts, ds), (t, d)| (format!("{ts}{t}"), format!("{ds}{d}")))
        .splet(|s| s.parse::<usize>().unwrap())
        .splot(calc_winners)
}
