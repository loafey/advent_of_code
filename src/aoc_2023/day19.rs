use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

use crate::utils::{bi_functors::BiFunctor, load_string};
use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    var: char,
    ordering: Ordering,
    num: usize,
    workflow: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkFlow {
    rules: Vec<Rule>,
    var: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Ratings {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
fn sum(rating: Ratings) -> usize {
    rating.x + rating.m + rating.a + rating.s
}

// a<2006:qkq
fn parse_rule(s: &str) -> Option<Rule> {
    let (first, workflow) = s.split_once(':')?;
    let workflow = workflow.to_string();
    let mut first = first.chars();
    let var = first.next()?;
    let ordering = match first.next()? {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        _ => unreachable!(),
    };
    let num = first.collect::<String>().parse::<usize>().ok()?;

    Some(Rule {
        var,
        ordering,
        num,
        workflow,
    })
}
fn parse_rating(s: &str) -> usize {
    s.split('=').nth(1).unwrap().parse::<usize>().unwrap()
}

fn input() -> (HashMap<String, WorkFlow>, Vec<Ratings>) {
    load_string("inputs/2023/day19.input")
        .split_once("\n\n")
        .unwrap()
        .splat(
            |t| {
                t.lines()
                    .map(|r| {
                        let mut splat = r.split(|c| matches!(c, '{' | '}'));
                        let name = splat.next().unwrap().to_string();
                        let rest = splat.next().unwrap();
                        let splat = rest.split(',');
                        let mut rules = Vec::new();
                        let mut var = String::new();
                        for part in splat {
                            if let Some(r) = parse_rule(part) {
                                rules.push(r);
                            } else {
                                var = part.to_string();
                            }
                        }
                        (name, WorkFlow { rules, var })
                    })
                    .collect::<HashMap<_, _>>()
            },
            |b| {
                b.lines()
                    .map(|b| {
                        let mut splat = b[1..b.len() - 1].split(',');
                        let x = parse_rating(splat.next().unwrap());
                        let m = parse_rating(splat.next().unwrap());
                        let a = parse_rating(splat.next().unwrap());
                        let s = parse_rating(splat.next().unwrap());
                        Ratings { x, m, a, s }
                    })
                    .collect::<Vec<_>>()
            },
        )
}

fn ok(rating: Ratings, wfs: &HashMap<String, WorkFlow>) -> bool {
    let mut current_pos = "in";
    'while_loop: while !matches!(current_pos, "A" | "R") {
        let wf = &wfs[current_pos];
        for rule in &wf.rules {
            match rule.var {
                'x' => {
                    if rating.x.cmp(&rule.num) == rule.ordering {
                        current_pos = &rule.workflow;
                        continue 'while_loop;
                    }
                }
                'm' => {
                    if rating.m.cmp(&rule.num) == rule.ordering {
                        current_pos = &rule.workflow;
                        continue 'while_loop;
                    }
                }
                'a' => {
                    if rating.a.cmp(&rule.num) == rule.ordering {
                        current_pos = &rule.workflow;
                        continue 'while_loop;
                    }
                }
                's' => {
                    if rating.s.cmp(&rule.num) == rule.ordering {
                        current_pos = &rule.workflow;
                        continue 'while_loop;
                    }
                }
                _ => unreachable!(),
            }
        }
        current_pos = &wf.var;
    }
    current_pos == "A"
}

pub fn part1() -> usize {
    let (workflows, ratings) = input();
    //workflows.iter().for_each(|(n, w)| println!("{n}: {w:?}"));
    //ratings.iter().for_each(|w| println!("{w:?}"));
    ratings
        .into_iter()
        .filter(|rating| ok(*rating, &workflows))
        .map(sum)
        .sum()
}

fn calc(wf: &str, wfs: &HashMap<String, WorkFlow>) -> usize {
    let wf = &wfs[wf];
    let mut rest = 3999 * 4;

    let mut total = 0;
    for rule in &wf.rules {
        let accept_num = match rule.ordering {
            Ordering::Less => rule.num - 1,
            Ordering::Greater => 3999 - rule.num,
            _ => unreachable!(),
        };

        total += match &rule.workflow[..] {
            "A" => accept_num,
            "R" => 0,
            xs => accept_num * calc(xs, wfs),
        };

        rest -= accept_num;
    }
    total += match &wf.var[..] {
        "A" => rest,
        "R" => 0,
        xs => rest * calc(xs, wfs),
    };
    total
}

pub fn part2() -> usize {
    let (workflows, _) = input();

    println!("167409079868000");
    calc("in", &workflows)
}
