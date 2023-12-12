use crate::utils::load_string;

#[memoize::memoize]
fn check(chars: Vec<char>, nums: Vec<usize>) -> usize {
    match (&chars[..], &nums[..]) {
        (s, []) if s.is_empty() || s.iter().all(|c| matches!(c, '.' | '?')) => 1,
        (_, []) => 0,
        ([], _) => 0,
        (css, [0, ns @ ..]) => check(css.to_vec(), ns.to_vec()),
        (css, nss) => match css[0] {
            '#' => {
                let mut nums = nums.to_vec();
                if nums[0] < css.len()
                    && css[..nums[0]].iter().all(|c| matches!(c, '#' | '?'))
                    && css[nums[0]] != '#'
                {
                    return check(css[nums[0] + 1..].to_vec(), nums[1..].to_vec());
                } else if nums[0] == css.len()
                    && nums.len() == 1
                    && css.iter().all(|c| matches!(c, '#' | '?'))
                {
                    return 1;
                }
                0
            }
            '.' => check(css[1..].to_vec(), nss.to_vec()),
            '?' => {
                let mut css = css.to_vec();
                css[0] = '#';
                let a = check(css.to_vec(), nss.to_vec());
                css[0] = '.';
                let b = check(css.to_vec(), nss.to_vec());
                a + b
            }
            _ => unreachable!(),
        },
    }
}

pub fn part1() -> usize {
    let binding = load_string("inputs/2023/day12.input");
    let inp = binding
        .lines()
        .map(|r| {
            let (row, nums) = r.split_once(' ').unwrap();
            let row = row.to_string();
            let mut nums = nums
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let row = row.chars().collect::<Vec<_>>();
            (nums, row)
        })
        .collect::<Vec<_>>();

    inp.into_iter().map(|(nums, row)| check(row, nums)).sum()
}

pub fn part2() -> usize {
    let binding = load_string("inputs/2023/day12.input");
    let inp = binding
        .lines()
        .map(|r| {
            let (row, nums) = r.split_once(' ').unwrap();
            let row = row.to_string();
            let mut nums = nums
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut row = row.chars().collect::<Vec<_>>();
            let num_clone = nums.clone();
            let row_clone = row.clone();
            for _ in 0..4 {
                row.push('?');
                row.append(&mut row_clone.clone());
                nums.append(&mut num_clone.clone());
            }
            (nums, row)
        })
        .collect::<Vec<_>>();
    inp.into_iter().map(|(nums, row)| check(row, nums)).sum()
}
