use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

use crate::utils::{bi_functors::BiFunctor, load_string};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct RatingsRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl std::fmt::Debug for RatingsRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ x: {:?}, m: {:?}, a: {:?}, s: {:?} }}",
            self.x, self.m, self.a, self.s
        )
    }
}

fn calc(wfstr: &str, rating: RatingsRange, wfs: &HashMap<String, WorkFlow>) -> usize {
    // print!("{wfstr}: ");
    if wfstr == "A" {
        // println!("Done\n");
        let x = rating.x;
        let m = rating.m;
        let a = rating.a;
        let s = rating.s;
        return (x.1 - x.0) * (m.1 - m.0) * (a.1 - a.0) * (s.1 - s.0);
    } else if wfstr == "R" {
        // println!("fail\n");
        return 0;
    }
    let wf = &wfs[wfstr];
    // println!("{wf:?}");
    // println!("Using rating: {rating:?}\n");

    let mut rest = rating;

    let mut total = 0;
    let mut orgy = rating;
    for rule in &wf.rules {
        let mut rating = orgy;
        match rule.var {
            'x' => match rule.ordering {
                Ordering::Greater => {
                    if rating.x.0 > rule.num {
                        continue;
                    }
                    if rating.x.1 >= rule.num {
                        rating.x.0 = rule.num + 1;
                        rest.x.1 = rule.num;
                        orgy.x = (0, 1)
                    }
                }
                Ordering::Less => {
                    if rating.x.1 < rule.num {
                        continue;
                    }
                    if rating.x.0 <= rule.num {
                        rating.x.1 = rule.num - 1;
                        rest.x.0 = rule.num;
                        orgy.x = (0, 1)
                    }
                }
                _ => unreachable!(),
            },
            'm' => match rule.ordering {
                Ordering::Greater => {
                    if rating.m.0 > rule.num {
                        continue;
                    }
                    if rating.m.1 >= rule.num {
                        rating.m.0 = rule.num + 1;
                        rest.m.1 = rule.num;
                        orgy.m = (0, 1)
                    }
                }
                Ordering::Less => {
                    if rating.m.1 < rule.num {
                        continue;
                    }
                    if rating.m.0 <= rule.num {
                        rating.m.1 = rule.num - 1;
                        rest.m.0 = rule.num;
                        orgy.m = (0, 1)
                    }
                }
                _ => unreachable!(),
            },
            'a' => match rule.ordering {
                Ordering::Greater => {
                    if rating.a.0 > rule.num {
                        continue;
                    }
                    if rating.a.1 >= rule.num {
                        rating.a.0 = rule.num + 1;
                        rest.a.1 = rule.num;
                        orgy.a = (0, 1)
                    }
                }
                Ordering::Less => {
                    if rating.a.1 < rule.num {
                        continue;
                    }
                    if rating.a.0 <= rule.num {
                        rating.a.1 = rule.num - 1;
                        rest.a.0 = rule.num;
                        orgy.a = (0, 1)
                    }
                }
                _ => unreachable!(),
            },
            's' => match rule.ordering {
                Ordering::Greater => {
                    if rating.s.0 > rule.num {
                        continue;
                    }
                    if rating.s.1 >= rule.num {
                        rating.s.0 = rule.num + 1;
                        rest.s.1 = rule.num;
                        orgy.s = (0, 1)
                    }
                }
                Ordering::Less => {
                    if rating.s.1 < rule.num {
                        continue;
                    }
                    if rating.s.0 <= rule.num {
                        rating.s.1 = rule.num - 1;
                        rest.s.0 = rule.num;
                        orgy.s = (0, 1)
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        total += calc(&rule.workflow, rating, wfs);
    }

    total += calc(&wf.var, rest, wfs);

    total
}

pub fn part2() -> usize {
    let (workflows, _) = input();

    println!("167409079868000");
    let rating = RatingsRange {
        x: (0, 4000),
        m: (0, 4000),
        a: (0, 4000),
        s: (0, 4000),
    };
    println!("Starting range: {rating:?}");
    calc("in", rating, &workflows)
}
