use std::collections::{BTreeMap, BTreeSet};

fn get_input() -> BTreeMap<char, BTreeSet<char>> {
    let mut dependencies = BTreeMap::new();
    include_str!("input/day7.input")
        .split('\n')
        .map(|l| {
            let mut split = l.split_whitespace().skip(1);
            let dep = split.next().unwrap().chars().next().unwrap();
            let depende = split.nth(5).unwrap().chars().next().unwrap();

            (depende, dep)
        })
        .for_each(|(depende, dep)| {
            dependencies.entry(dep).or_insert_with(BTreeSet::new);
            dependencies.entry(depende).or_insert_with(BTreeSet::new);
            if let Some(d) = dependencies.get_mut(&depende) {
                d.insert(dep);
            }
        });

    dependencies
}

pub fn part1() -> String {
    let mut dependencies = get_input();

    let mut execution = String::new();
    while !dependencies.is_empty() {
        let mut done_instruction = None;
        for (c, deps) in dependencies.iter() {
            if deps.is_empty() {
                execution.push(*c);
                done_instruction = Some(*c);
                break;
            }
        }

        if let Some(di) = done_instruction {
            dependencies.remove(&di);
            for (_, deps) in dependencies.iter_mut() {
                deps.remove(&di);
            }
        }
    }
    execution
}
pub fn part2() -> i32 {
    0
}
