use crate::utils::load_string;

fn check(chars: &mut [char], nums: &mut [usize]) -> usize {
    // println!("{chars:?} {nums:?}",);
    match (chars, nums) {
        (s, []) if s.is_empty() || s.iter().all(|c| *c == '.') => 1,
        (_, []) => 0,
        ([], _) => 0,
        (css, [0, ns @ ..]) => check(css, ns),
        (css, nss) => match css[0] {
            '#' => {
                nss[0] -= 1;
                check(&mut css[1..], nss)
            }
            '.' => check(&mut css[1..], nss),
            '?' => {
                css[0] = '#';
                let a = check(css, nss);
                css[0] = '.';
                let b = check(css, nss);
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

    inp.into_iter()
        .map(|(mut nums, mut row)| {
            print!("{row:?} {nums:?}");
            let r = check(&mut row, &mut nums);
            println!(": {r}");
            r
        })
        .sum()
}

pub fn part2() -> usize {
    0
    // let binding = load_string("inputs/2023/day12.input");
    // let inp = binding
    //     .lines()
    //     .map(|r| {
    //         let (row, nums) = r.split_once(' ').unwrap();
    //         let row = row.to_string();
    //         let mut nums = nums
    //             .split(',')
    //             .map(|s| s.parse::<usize>().unwrap())
    //             .collect::<Vec<_>>();
    //         let mut row = row.chars().collect::<Vec<_>>();
    //         let num_clone = nums.clone();
    //         let row_clone = row.clone();
    //         for _ in 0..4 {
    //             row.push('?');
    //             row.append(&mut row_clone.clone());
    //             nums.append(&mut num_clone.clone());
    //         }
    //         (nums, row)
    //     })
    //     .collect::<Vec<_>>();
    // inp.into_par_iter()
    //     .map(|(nums, row)| perm(row, &nums))
    //     .sum()
}
