use std::collections::HashMap;
use utils::load_string;

pub fn part1() -> i32 {
    let inp = load_string("inputs/2018/day2.input");
    let mut doubles = 0;
    let mut triples = 0;
    inp.split('\n').for_each(|s| {
        let mut map = HashMap::new();
        s.chars().for_each(|c| {
            if let Some(i) = map.get_mut(&c) {
                *i += 1;
            } else {
                map.insert(c, 1);
            }
        });

        let mut already_doubled = false;
        let mut already_tripled = false;
        for (_, v) in map {
            if v == 2 && !already_doubled {
                doubles += 1;
                already_doubled = true;
            } else if v == 3 && !already_tripled {
                triples += 1;
                already_tripled = true;
            }

            if already_doubled && already_tripled {
                break;
            }
        }
    });
    doubles * triples
}
pub fn part2() -> String {
    let inp = load_string("inputs/2018/day2.input");
    let split = inp.split('\n').collect::<Vec<_>>();

    let mut result_string = String::new();

    'outer: for s in split.iter() {
        for c in split.iter().map(|s| s.chars()) {
            let mut count = s.len();

            for (i, c) in c.clone().enumerate() {
                if s[i..i + 1] == c.to_string() {
                    count -= 1;
                }
            }
            if count == 1 {
                for c in c {
                    if s.contains(c) {
                        result_string.push(c);
                    }
                }
                break 'outer;
            }
        }
    }
    result_string
}
