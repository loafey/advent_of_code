use std::collections::HashMap;

type Id<'l> = &'l str;

#[derive(Debug)]
enum Command<'l> {
    Ls(Vec<LsOutput<'l>>),
    Cd(Id<'l>),
}
impl<'l> From<&'l str> for Command<'l> {
    fn from(value: &'l str) -> Self {
        match &value.split_whitespace().collect::<Vec<_>>()[..] {
            ["cd", arg] => Command::Cd(arg),
            ["ls", rest @ ..] => {
                Command::Ls(rest.chunks(2).map(LsOutput::from).collect::<Vec<_>>())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum LsOutput<'l> {
    Directory(Id<'l>),
    File(usize),
}
impl<'l> From<&[&'l str]> for LsOutput<'l> {
    fn from(val: &[&'l str]) -> Self {
        let f = val[0];
        let name = val[1];
        match f.parse::<usize>() {
            Ok(s) => LsOutput::File(s),
            Err(_) => LsOutput::Directory(name),
        }
    }
}

pub fn part1() -> usize {
    let input = include_str!("input/day7.input");
    let mut stack = Vec::new();
    let mut memory = HashMap::new();
    for p in input
        .split("$ ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Command::from)
    {
        match p {
            Command::Ls(v) => {
                memory.insert(stack[stack.len() - 1], v);
            }
            Command::Cd(dir) => match dir {
                ".." => {
                    stack.pop();
                }
                dir => {
                    stack.push(dir);
                }
            },
        }
    }
    println!("\n");
    let mut sizes = HashMap::new();
    let mut stack = vec!["/"];
    let mut pairs = Vec::new();
    while let Some(top) = stack.pop() {
        for f in &memory[top] {
            match f {
                LsOutput::File(s) => {
                    if let Some(r) = sizes.get_mut(top) {
                        *r += s;
                    } else {
                        sizes.insert(top, *s);
                    }
                }
                LsOutput::Directory(dir) => {
                    stack.push(dir);
                    pairs.push((top, *dir));
                    stack.iter().for_each(|d| pairs.push((d, dir)));
                    if !sizes.contains_key(dir) {
                        sizes.insert(dir, 0);
                    }
                }
            }
        }
        println!("{stack:?}")
    }
    let mut sizes_clone = sizes.clone();
    println!(
        "{}",
        sizes.values().filter(|u| **u <= 100000).sum::<usize>()
    );
    for (whom, whale) in pairs {
        //println!("{whom} {whale}");
        *sizes_clone.get_mut(whom).unwrap() += sizes[whale];
    }

    sizes_clone.values().filter(|u| **u <= 100000).sum()
}

pub fn part2() -> usize {
    0
}
