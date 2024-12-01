use utils::load_string;

pub fn inputs() -> Vec<Vec<i64>> {
    load_string("inputs/2023/day9.input")
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn calc_diffs(mut inp: Vec<i64>) -> Vec<Vec<i64>> {
    let mut dif = vec![Vec::new()];
    let mut current_dif = 0;
    loop {
        let mut last = inp[0];
        for i in inp.iter().skip(1) {
            dif[current_dif].push(i - last);
            last = *i;
        }
        if dif[current_dif].iter().all(|a| *a == 0) {
            break;
        } else {
            dif.push(Vec::new());
            inp = dif[current_dif].clone();
            current_dif += 1;
        }
    }

    let mut current_dif = dif.len() - 1;
    while current_dif > 0 {
        if dif[current_dif][0] == 0 {
            dif[current_dif].insert(0, 0)
        }
        let val = dif[current_dif - 1][0] - dif[current_dif][0];
        dif[current_dif - 1].insert(0, val);
        current_dif -= 1;
    }
    dif
}

pub fn part1() -> i64 {
    inputs()
        .into_iter()
        .map(|i| {
            let last = *i.last().unwrap();
            let diffs = calc_diffs(i);
            let mut next = 0;
            for diff in diffs.into_iter().rev() {
                next += diff.last().unwrap();
            }
            last + next
        })
        .sum()
}
pub fn part2() -> i64 {
    inputs()
        .into_iter()
        .map(|i| {
            let first = *i.first().unwrap();
            let diffs = calc_diffs(i);
            let next = *diffs[0].first().unwrap();
            first - next
        })
        .sum()
}
