#[derive(Clone, Copy)]
enum Data {
    File(i64, usize),
    Empty(usize),
}
impl Data {
    pub fn len(&self) -> usize {
        match self {
            Data::File(_, l) => *l,
            Data::Empty(l) => *l,
        }
    }
}

fn solve(breakie: bool) -> i64 {
    let chas = include_str!("../inputs/2024/day9.input")
        .chars()
        .filter(|s| s.is_numeric())
        .map(|s| format!("{s}").parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut map = Vec::new();

    let mut alt = false;
    let mut id = 0;
    for d in chas {
        if breakie {
            if alt {
                for _ in 0..d {
                    map.push(Data::Empty(1));
                }
            } else {
                for _ in 0..d {
                    map.push(Data::File(id, 1));
                }
                id += 1;
            }
        } else if alt {
            map.push(Data::Empty(d as usize));
        } else {
            map.push(Data::File(id, d as usize));
            id += 1;
        }
        alt = !alt
    }

    let mut left_most = 0;
    let mut right_most = map.len() - 1;
    while left_most < map.len() {
        if right_most == 0 {
            break;
        }
        if !matches!(map[left_most], Data::Empty(_)) {
            left_most += 1;
            continue;
        }
        if matches!(map[right_most], Data::Empty(_)) {
            right_most -= 1;
            continue;
        }
        let a = map[left_most];
        let b = map[right_most];
        if a.len() >= b.len() && left_most < right_most {
            let diff = a.len() - b.len();
            map[left_most] = b;
            map[right_most] = Data::Empty(b.len());
            if diff > 0 {
                map.insert(left_most + 1, Data::Empty(diff));
            }
            left_most = 0;
            // right_most = map.len() - 1;
        } else {
            left_most += 1;
        }
        if left_most >= map.len() {
            left_most = 0;
            right_most -= 1;
        }
    }

    let mut sum = 0;
    let mut gi = 0;
    for v in map {
        match v {
            Data::File(a, d) => {
                for i in 0..d {
                    sum += (gi + i as i64) * a;
                }
                gi += d as i64;
            }
            Data::Empty(d) => gi += d as i64,
        }
    }
    sum
}

pub fn part1() -> i64 {
    solve(true)
}
pub fn part2() -> i64 {
    solve(false)
}
