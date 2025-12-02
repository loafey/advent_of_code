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
            let str = format!("{i}");
            let (a, b) = str.split_at(str.len() / 2);
            if a == b {
                sum += i;
            }
        }
    }
    sum
}

// this is not very nice :)
pub fn part2() -> u64 {
    let mut sum = 0;
    for (a, b) in input() {
        'outer: for i in a..=b {
            let str = format!("{i}");
            for p in 1..((str.len() / 2) + 1) {
                let mut bucket = Vec::new();
                let mut temp = String::new();
                for (ind, c) in str.chars().enumerate() {
                    temp.push(c);
                    if (ind + 1) % p == 0 {
                        bucket.push(temp);
                        temp = String::new();
                    }
                }
                if !temp.is_empty() {
                    bucket.push(temp);
                }
                let first = &bucket[0];
                if bucket.iter().all(|v| v == first) {
                    sum += i;
                    continue 'outer;
                }
            }
        }
    }
    sum
}
