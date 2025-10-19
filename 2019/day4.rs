fn split_num(i: usize) -> [usize; 6] {
    [
        (i / 100000) % 10,
        (i / 10000) % 10,
        (i / 1000) % 10,
        (i / 100) % 10,
        (i / 10) % 10,
        i % 10,
    ]
}
fn meets_requirements(i: usize) -> bool {
    fn has_adjacents(i: [usize; 6]) -> bool {
        for ind in 0..i.len() - 1 {
            if i[ind] == i[ind + 1] {
                return true;
            }
        }
        false
    }
    fn no_decrease(i: [usize; 6]) -> bool {
        for ind in 0..i.len() - 1 {
            if i[ind] > i[ind + 1] {
                return false;
            }
        }
        true
    }
    let i = split_num(i);

    has_adjacents(i) && no_decrease(i)
}

pub fn part1() -> usize {
    (172930..683082).filter(|p| meets_requirements(*p)).count()
}

pub fn part2() -> i32 {
    panic!()
}
