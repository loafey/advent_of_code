fn solve((ax, ay): (i64, i64), (bx, by): (i64, i64), (gx, gy): (i64, i64)) -> Option<(i64, i64)> {
    let m = (ax * gy - gx * ay) / (ax * by - bx * ay);
    let n = (gx - m * bx) / ax;
    if n * ax + m * bx == gx && n * ay + m * by == gy {
        Some((m, n))
    } else {
        None
    }
}

fn parse_and_solve(m: i64) -> i64 {
    let input = include_str!("../inputs/2024/day13.input")
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|r| {
            let s = [' ', 'X', '+', ',', 'Y', ':', 'A', 'B', '='];
            let mut l = r.lines();
            macro_rules! p {
                () => {{
                    let a = l.next().unwrap();
                    let mut a = a.split(s).filter(|s| !s.is_empty()).skip(1);
                    let a1 = a.next().unwrap().parse::<i64>().unwrap();
                    let a2 = a.next().unwrap().parse::<i64>().unwrap();
                    (a1, a2)
                }};
            }
            (p!(), p!(), p!())
        })
        .collect::<Vec<_>>();

    input
        .into_iter()
        .filter_map(|(a, b, (gx, gy))| solve(a, b, (gx + m, gy + m)))
        .map(|(b, a)| a * 3 + b)
        .sum()
}

pub fn part1() -> i64 {
    parse_and_solve(0)
}
pub fn part2() -> i64 {
    parse_and_solve(10000000000000)
}
