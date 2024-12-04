pub fn part1() -> i64 {
    let inp = include_str!("../inputs/2024/day4.input");
    let m = inp
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (y, r) in m.iter().enumerate() {
        for (x, _) in r.iter().enumerate().filter(|(_, c)| **c == 'X') {
            let mut checks = Vec::new();
            if x + 3 < r.len() {
                let c = m[y][x..x + 4].to_vec();
                checks.push(c);
            }
            if x > 2 {
                let mut c = m[y][x - 3..=x].to_vec();
                c.reverse();
                checks.push(c);
            }
            if y > 2 {
                let c = vec![m[y][x], m[y - 1][x], m[y - 2][x], m[y - 3][x]];
                checks.push(c);
            }
            if y + 3 < m.len() {
                let c = vec![m[y][x], m[y + 1][x], m[y + 2][x], m[y + 3][x]];
                checks.push(c);
            }
            if y > 2 && x > 2 {
                let d = vec![m[y][x], m[y - 1][x - 1], m[y - 2][x - 2], m[y - 3][x - 3]];
                checks.push(d);
            }

            if y > 2 && x + 3 < r.len() {
                let c = vec![m[y][x], m[y - 1][x + 1], m[y - 2][x + 2], m[y - 3][x + 3]];
                checks.push(c);
            }

            if y + 3 < m.len() && x > 2 {
                let c = vec![m[y][x], m[y + 1][x - 1], m[y + 2][x - 2], m[y + 3][x - 3]];
                checks.push(c);
            }

            if y + 3 < m.len() && x + 3 < r.len() {
                let d = vec![m[y][x], m[y + 1][x + 1], m[y + 2][x + 2], m[y + 3][x + 3]];
                checks.push(d);
            }

            for c in checks {
                ans += (c == ['X', 'M', 'A', 'S']) as i64
            }
        }
    }
    ans
}

pub fn part2() -> i64 {
    let inp = include_str!("../inputs/2024/day4.input");
    let m = inp
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (y, r) in m.iter().enumerate() {
        for (x, _) in r.iter().enumerate().filter(|(_, c)| **c == 'A') {
            if x != 0 && y != 0 && x < r.len() - 1 && y < m.len() - 1 {
                let f1 = [m[y - 1][x - 1], m[y][x], m[y + 1][x + 1]];
                let f2 = [m[y + 1][x + 1], m[y][x], m[y - 1][x - 1]];
                let s1 = [m[y - 1][x + 1], m[y][x], m[y + 1][x - 1]];
                let s2 = [m[y + 1][x - 1], m[y][x], m[y - 1][x + 1]];

                if (f1 == ['M', 'A', 'S'] || f2 == ['M', 'A', 'S'])
                    && (s1 == ['M', 'A', 'S'] || s2 == ['M', 'A', 'S'])
                {
                    ans += 1;
                }
            }
        }
    }
    ans
}
