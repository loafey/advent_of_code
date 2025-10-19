pub fn part1() -> i64 {
    let input = include_str!("../inputs/2024/day25.input");
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.lines()
                .map(|c| c.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .for_each(|k| {
            let mut shape = Vec::new();
            if k[0][0] == '#' {
                for c in 0..k[0].len() {
                    let mut len = 0;
                    for y in &k {
                        if y[c] == '#' {
                            len += 1;
                        } else {
                            break;
                        }
                    }
                    shape.push(len - 1);
                }
                locks.push(shape);
            } else {
                for c in 0..k[0].len() {
                    let mut len = 0;
                    for y in k.iter().rev() {
                        if y[c] == '#' {
                            len += 1;
                        } else {
                            break;
                        }
                    }
                    shape.push(len - 1);
                }
                keys.push(shape);
            }
        });

    let mut sum = 0;
    for l in locks {
        for k in &keys {
            if !l
                .iter()
                .zip(k)
                .map(|(a, b)| (*a, *b))
                .map(|(a, b)| (a + b))
                .any(|c| c > 5)
            {
                sum += 1;
                // eprintln!("Lock {l:?} and key {k:?}: don't overlap")
            } else {
                // eprintln!("Lock {l:?} and key {k:?}: overlap")
            }
        }
    }
    sum
}
pub fn part2() -> i64 {
    0
}
