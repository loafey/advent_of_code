use memoize::memoize;
use std::{collections::HashMap as Map, ops::Deref, rc::Rc};

#[derive(Debug)]
enum ParseState {
    Bag1,
    Bag2,
    Num,
    Skip(usize),
}

#[derive(Clone, PartialEq, Eq)]
struct Wrapper(Rc<Map<String, Map<String, usize>>>);

impl std::hash::Hash for Wrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        0.hash(state);
    }
}
impl Deref for Wrapper {
    type Target = Map<String, Map<String, usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parser(input: &str) -> Wrapper {
    let mut res = Map::new();

    for line in input.lines() {
        let mut key = String::new();
        let mut map = Map::new();
        let mut num = 0;
        let mut cur_word = String::new();
        let mut state = ParseState::Bag1;
        let mut first = true;
        for word in line
            .split(|c: char| c.is_whitespace() || matches!(c, '.' | ','))
            .filter(|s| !s.trim().is_empty())
        {
            if word == "no" {
                break;
            }
            match state {
                ParseState::Bag1 => {
                    cur_word += word;
                    state = ParseState::Bag2;
                }
                ParseState::Bag2 => {
                    cur_word += &format!(" {}", word);
                    if first {
                        state = ParseState::Skip(1);
                        first = false;
                        key = cur_word;
                        cur_word = String::new();
                    } else {
                        state = ParseState::Skip(0);
                        map.insert(cur_word, num);

                        cur_word = String::new();
                    }
                }
                ParseState::Num => {
                    num = word.parse::<usize>().unwrap();
                    state = ParseState::Bag1;
                }
                ParseState::Skip(0) => state = ParseState::Num,
                ParseState::Skip(n) => state = ParseState::Skip(n - 1),
            }
        }
        res.insert(key, map);
    }

    Wrapper(Rc::new(res))
}

#[memoize]
fn contains_shiny(which: String, map: Wrapper) -> usize {
    let Some(entry) = map.get(&which) else {
        return 0;
    };

    (entry.contains_key("shiny gold")
        || entry
            .iter()
            .map(|(entry, _)| contains_shiny(entry.clone(), map.clone()))
            .sum::<usize>()
            > 0) as usize
}

#[memoize]
fn child_count(which: String, map: Wrapper) -> usize {
    let Some(top) = map.get(&which) else {
        return 0;
    };

    1 + top
        .iter()
        .map(|(bag, count)| {
            let child_count = child_count(bag.clone(), map.clone());
            count * child_count
        })
        .sum::<usize>()
}

pub fn part1() -> usize {
    let input = include_str!("../../inputs/2020/day7.input");
    let parsed = parser(input);
    parsed
        .keys()
        .cloned()
        .map(|s| contains_shiny(s.to_string(), parsed.clone()))
        .sum()
}

pub fn part2() -> usize {
    let input = include_str!("../../inputs/2020/day7.input");
    let parsed = parser(input);
    child_count("shiny gold".to_string(), parsed) - 1
}
