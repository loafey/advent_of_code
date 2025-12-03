fn input() -> impl Iterator<Item = Vec<u64>> {
    include_str!("../inputs/2025/day3.input")
        .lines()
        .map(|c| c.chars().map(|c| c as u64 - 0x30).collect::<Vec<_>>())
}

fn solve(mut count: usize, data: &[u64]) -> u64 {
    let mut num = 0;
    let mut start = 0;
    while count != 0 {
        count -= 1;
        let mut max = (0, 0);
        for new in data[start..data.len() - count].iter().copied().enumerate() {
            if new.1 > max.1 {
                max = new;
            }
        }
        start += max.0 + 1;
        num += max.1 * 10u64.pow(count as u32);
    }
    num
}

pub fn part1() -> u64 {
    input().map(|bank| solve(2, &bank[..])).sum()
}

pub fn part2() -> u64 {
    input().map(|bank| solve(12, &bank[..])).sum()
}
