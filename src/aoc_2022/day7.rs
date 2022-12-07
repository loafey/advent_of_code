use crate::utils::hmap_insert_vec;
use std::collections::HashMap;

type Id = &'static str;

enum Command {
    Ls(Vec<LsOutput>),
    Cd(Id),
}
impl From<Id> for Command {
    fn from(value: Id) -> Self {
        match &value.split_whitespace().collect::<Vec<_>>()[..] {
            ["cd", arg] => Command::Cd(arg),
            ["ls", rest @ ..] => {
                Command::Ls(rest.chunks(2).map(LsOutput::from).collect::<Vec<_>>())
            }
            _ => unreachable!(),
        }
    }
}

enum LsOutput {
    Directory(Id),
    File(usize),
}
impl LsOutput {
    fn is_file(&self) -> bool {
        match self {
            LsOutput::Directory(_) => false,
            LsOutput::File(_) => true,
        }
    }
    fn size(&self) -> usize {
        match self {
            LsOutput::Directory(_) => panic!(),
            LsOutput::File(u) => *u,
        }
    }
    fn dir_name(&self) -> Option<&str> {
        match self {
            LsOutput::Directory(d) => Some(d),
            LsOutput::File(_) => None,
        }
    }
}
impl From<&[Id]> for LsOutput {
    fn from(val: &[Id]) -> Self {
        let f = val[0];
        let name = val[1];
        match f.parse::<usize>() {
            Ok(s) => LsOutput::File(s),
            Err(_) => LsOutput::Directory(name),
        }
    }
}

fn solver() -> HashMap<String, usize> {
    let input = include_str!("input/day7.input");
    let mut stack = Vec::new();
    let mut memory = HashMap::new();
    for p in input
        .split("$ ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Command::from)
    {
        let stacks = stack.join("");
        match p {
            Command::Ls(v) => {
                hmap_insert_vec(&mut memory, stacks, v);
            }
            Command::Cd(dir) => match dir {
                ".." => {
                    stack.pop();
                }
                dir => {
                    stack.push(dir);
                    hmap_insert_vec(&mut memory, format!("{stacks}{dir}"), Vec::new())
                }
            },
        }
    }
    let mut annotated = HashMap::new();
    while !memory.is_empty() {
        let mut to_remove = Vec::new();
        memory
            .iter()
            .filter(|(_, v)| v.iter().map(|l| l.is_file()).all(|x| x))
            .for_each(|(key, l)| {
                annotated.insert(key.clone(), l.iter().map(|l| l.size()).sum::<usize>());
                to_remove.push(key.clone());
            });
        to_remove.into_iter().for_each(|k| {
            memory.remove(&k);
        });
        memory.iter_mut().for_each(|(base, v)| {
            v.iter_mut().for_each(|ls| {
                if let Some(dir_name) = ls.dir_name() {
                    let dir_name = format!("{base}{dir_name}");
                    if annotated.contains_key(&dir_name) {
                        *ls = LsOutput::File(*annotated.get(&dir_name).unwrap());
                    }
                }
            });
        });
    }
    annotated
}

pub fn part1() -> usize {
    solver()
        .into_iter()
        .filter(|(_, k)| *k <= 100000)
        .map(|(_, u)| u)
        .sum()
}

pub fn part2() -> usize {
    let solver = solver();
    let total = 70000000;
    let needed = 30000000;
    let size = solver["/"];
    solver
        .into_iter()
        .filter(|(_, current)| total - needed >= size - current)
        .map(|(_, c)| c)
        .min()
        .unwrap()
}
