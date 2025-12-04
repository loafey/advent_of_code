use utils::MatrixGet;

enum Item {
    Empty,
    Roll,
}

fn input() -> Vec<Vec<Item>> {
    include_str!("../inputs/2025/day4.input")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Item::Empty,
                    '@' => Item::Roll,
                    _ => unimplemented!(),
                })
                .collect()
        })
        .collect()
}

fn remove_rolls(matrix: &mut Vec<Vec<Item>>) -> u64 {
    let mut ans = 0;
    let mut removable = Vec::new();
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if !matches!(matrix[y][x], Item::Roll) {
                continue;
            }
            let col = vec![
                matrix.mget(y, x, -1, -1),
                matrix.mget(y, x, -1, 0),
                matrix.mget(y, x, -1, 1),
                matrix.mget(y, x, 0, -1),
                matrix.mget(y, x, 0, 1),
                matrix.mget(y, x, 1, -1),
                matrix.mget(y, x, 1, 0),
                matrix.mget(y, x, 1, 1),
            ];
            if col
                .into_iter()
                .filter(|p| matches!(p, Some(Item::Roll)))
                .count()
                < 4
            {
                removable.push((y, x));
                ans += 1
            }
        }
    }
    for (y, x) in removable {
        matrix[y][x] = Item::Empty;
    }
    ans
}

pub fn part1() -> u64 {
    let mut matrix = input();
    remove_rolls(&mut matrix)
}

pub fn part2() -> u64 {
    let mut matrix = input();
    let mut ans = 0;
    loop {
        let removed = remove_rolls(&mut matrix);
        ans += removed;
        if removed == 0 {
            break;
        }
    }
    ans
}
