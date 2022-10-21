use crate::utils::parse_next;
use chrono::{Duration, NaiveDate};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    ops::{Range, RangeInclusive},
};

fn parse_sleep_ranges() -> HashMap<i32, Vec<Range<i32>>> {
    enum Event {
        GuardBegin(i32),
        FallAsleep,
        WakeUp,
    }

    let mut sleep_ranges: HashMap<_, Vec<_>> = HashMap::new();

    let mut current_guard = 0;
    let mut current_start = 0;

    let mut dates: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut l = include_str!("input/day4.input")
        .split('\n')
        .map(|s| {
            let mut split = s.split_whitespace();
            let mut date = split.next().unwrap()[1..].parse::<NaiveDate>().unwrap();
            let time = {
                let s = split.next().unwrap();
                let time = &s[0..s.len() - 1]; //xx:xx
                let mut time_split = time.split(':');
                let hour = parse_next::<i32>(&mut time_split);
                let minutes = parse_next::<i32>(&mut time_split);
                if hour == 23 {
                    date += Duration::days(1);
                    minutes - 60
                } else {
                    minutes
                }
            };
            let e = match split.next().unwrap() {
                "Guard" => Event::GuardBegin(split.next().unwrap()[1..].parse::<i32>().unwrap()),
                "falls" => Event::FallAsleep,
                "wakes" => Event::WakeUp,
                _ => unreachable!(),
            };
            (date, time, e) // combine these!
        })
        .for_each(|(date, time, e)| {
            if let Some(d) = dates.get_mut(&date) {
                d.push((time, e));
            } else {
                dates.insert(date, vec![(time, e)]);
            }
        });

    dates
        .iter_mut()
        .for_each(|(_, k)| k.sort_by_key(|(i, _)| *i));

    dates
        .into_iter()
        .flat_map(|(_, k)| k.into_iter())
        .for_each(|(t, e)| match e {
            Event::GuardBegin(g) => current_guard = g,
            Event::FallAsleep => current_start = t,
            Event::WakeUp => {
                if let Some(g) = sleep_ranges.get_mut(&current_guard) {
                    g.push(current_start..t)
                } else {
                    sleep_ranges.insert(current_guard, vec![current_start..t]);
                }
            }
        });

    sleep_ranges
}

pub fn part1() -> i32 {
    let mut sleep_ranges = parse_sleep_ranges();

    let most_asleep = {
        let mut guards = BTreeMap::new();
        for (guard, sleep_times) in &sleep_ranges {
            guards.insert(
                guard,
                sleep_times.iter().map(|s| s.end - s.start).sum::<i32>(),
            );
        }
        let mut guards = guards
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<BTreeMap<_, _>>();
        *guards.pop_last().unwrap().1
    };

    let big_minute = {
        let mut minutes = [0; 60];
        for range in sleep_ranges.remove(&most_asleep).unwrap() {
            for v in range {
                minutes[v as usize] += 1;
            }
        }
        let mut largest_val = 0;
        let mut largest_index = 0;
        (0..minutes.len()).for_each(|i| {
            if minutes[i] > largest_val {
                largest_val = minutes[i];
                largest_index = i;
            }
        });
        largest_index
    };

    println!("{} {}", most_asleep, big_minute);

    most_asleep * big_minute as i32
}

pub fn part2() -> i32 {
    let sleep_ranges = parse_sleep_ranges()
        .into_iter()
        .map(|(k, r)| {
            let mut minutes = [0; 60];
            for range in r {
                for v in range {
                    minutes[v as usize] += 1;
                }
            }
            (k, minutes)
        })
        .collect::<Vec<_>>();

    let mut k = sleep_ranges
        .into_iter()
        .map(|(g, v)| {
            let mut biggest = 0;
            let mut biggest_index = 0;
            (0..v.len()).for_each(|i| {
                if v[i] > biggest {
                    biggest = v[i];
                    biggest_index = i;
                }
            });
            (biggest, g * biggest_index as i32)
        })
        .collect::<BTreeMap<_, _>>();

    println!("{k:?}");
    k.pop_last().unwrap().1
}
