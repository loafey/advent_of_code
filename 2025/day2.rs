use utils::NumExt;

fn input() -> impl Iterator<Item = (u64, u64)> {
    include_str!("../inputs/2025/day2.input")
        .split(',')
        .map(|s| {
            let (a, b) = s.trim().split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
}

pub fn part1() -> u64 {
    let mut sum = 0;
    for (a, b) in input() {
        for i in a..=b {
            let (a, b) = i.split();
            if a == b {
                sum += i;
            }
        }
    }
    sum
}

// this is not very nice :)
pub fn part2() -> u64 {
    let sum = input()
        .flat_map(|(a, b)| {
            (a..=b).filter(|i| {
                let s = i.to_string();
                (1..=(s.len() / 2))
                    .any(|p| s.matches(&s[..p]).map(str::len).sum::<usize>() == s.len())
            })
        })
        .sum();
    sum
}
