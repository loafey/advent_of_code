use crate::utils::parse_next;
use chrono::{Duration, NaiveDate};
use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

fn parse_sleep_ranges() -> HashMap<i32, Vec<Range<i32>>> {
    enum Event {
        GuardBegin(i32),
        FallAsleep,
        WakeUp,
    }

    let mut current_guard = 0;
    let mut current_start = 0;
    let mut dates: BTreeMap<_, Vec<_>> = BTreeMap::new();
    include_str!("input/day4.input").split('\n').for_each(|s| {
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
        if let Some(d) = dates.get_mut(&date) {
            d.push((time, e));
        } else {
            dates.insert(date, vec![(time, e)]);
        }
    });

    dates
        .iter_mut()
        .for_each(|(_, k)| k.sort_by_key(|(i, _)| *i));

    let mut sleep_ranges: HashMap<_, Vec<_>> = HashMap::new();
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

fn create_sleep_stats(sleep_ranges: Vec<Range<i32>>) -> (i32, usize) {
    let mut minutes = [0; 60];
    for range in sleep_ranges {
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
    (largest_val, largest_index)
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
        *guards
            .into_iter()
            .map(|(k, v)| (v, k))
            .reduce(|accum, other| if accum.0 > other.0 { accum } else { other })
            .unwrap()
            .1
    };

    let big_minute = create_sleep_stats(sleep_ranges.remove(&most_asleep).unwrap()).1;

    most_asleep * big_minute as i32
}

pub fn part2() -> i32 {
    parse_sleep_ranges()
        .into_iter()
        .map(|(guard, sleep_ranges)| {
            let (biggest, biggest_index) = create_sleep_stats(sleep_ranges);
            (biggest, guard * biggest_index as i32)
        })
        .reduce(|accum, other| if accum.0 > other.0 { accum } else { other })
        .unwrap()
        .1
}
