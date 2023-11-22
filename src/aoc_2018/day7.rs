use crate::utils::load_string;
use std::collections::{BTreeMap, BTreeSet};

fn get_input() -> BTreeMap<char, BTreeSet<char>> {
    let mut dependencies = BTreeMap::new();
    load_string("inputs/2018/day7.input")
        .lines()
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

fn char_to_second(c: char) -> i32 {
    c as u8 as i32 - 4
}

#[derive(Clone, Copy)]
struct Worker {
    task: Option<char>,
    time: i32,
}
impl Worker {
    pub fn has_task(&self) -> bool {
        self.task.is_some()
    }
    pub fn is_done(&self) -> bool {
        self.time <= 0
    }
    pub fn new() -> Self {
        Self {
            task: None,
            time: 0,
        }
    }
}

pub fn part2() -> usize {
    let mut dependencies = get_input();
    let mut execution = String::new();
    let mut workers = [Worker::new(); 5];

    let mut time = 0;
    while !dependencies.is_empty() || workers.iter().map(|w| w.has_task()).any(|mk| mk) {
        for w in &mut workers {
            if w.is_done() {
                if let Some(di) = w.task {
                    execution.push(di);
                    dependencies.remove(&di);
                    for (_, deps) in dependencies.iter_mut() {
                        deps.remove(&di);
                    }
                }
                w.task = None;
            }
        }

        for w in &mut workers {
            let mut to_remove = None;
            for (c, deps) in dependencies.iter() {
                if deps.is_empty() && !w.has_task() {
                    w.task = Some(*c);
                    w.time = char_to_second(*c);
                    to_remove = Some(*c);
                    break;
                }
            }

            if let Some(k) = to_remove {
                dependencies.remove(&k);
            }
        }

        // let workers_work = workers
        //     .iter()
        //     .map(|w| w.task.unwrap_or('.').to_string())
        //     .collect::<Vec<_>>()
        //     .join("\t");
        // println!("{time}\t{}\t{execution}", workers_work);
        // std::thread::sleep(std::time::Duration::from_secs_f32(0.1));

        for w in &mut workers {
            w.time -= 1;
        }
        time += 1;
    }

    time - 1
}
