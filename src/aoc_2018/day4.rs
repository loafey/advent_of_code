use std::collections::{BTreeMap, HashMap};

use crate::utils::parse_next;

pub fn part1() -> i32 {
    enum Event {
        GuardBegin(i32),
        FallAsleep,
        WakeUp,
    }

    let mut current_guard = 0;
    let mut current_start = 0;
    let mut sleep_ranges: HashMap<_, Vec<_>> = HashMap::new();
    include_str!("input/day4.input")
        .split('\n')
        .map(|s| {
            let mut split = s.split_whitespace().skip(1);
            let time = {
                let s = split.next().unwrap();
                let time = &s[0..s.len() - 1]; //xx:xx
                let mut time_split = time.split(':');
                let hour = parse_next::<i32>(&mut time_split);
                let minutes = parse_next::<i32>(&mut time_split);
                if hour == 23 {
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
            (time, e) // combine these!
        })
        .for_each(|(t, e)| match e {
            Event::GuardBegin(g) => current_guard = g,
            Event::FallAsleep => current_start = t,
            Event::WakeUp => {
                if let Some(g) = sleep_ranges.get_mut(&current_guard) {
                    g.push(current_start..=t)
                } else {
                    sleep_ranges.insert(current_guard, vec![current_start..=t]);
                }
            }
        });

    let most_asleep = {
        let mut guards = BTreeMap::new();
        for (guard, sleep_times) in &sleep_ranges {
            guards.insert(
                guard,
                sleep_times.iter().map(|s| s.end() - s.start()).sum::<i32>(),
            );
        }
        let mut guards = guards
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<BTreeMap<_, _>>();
        *guards.pop_last().unwrap().1
    };

    println!("{:?}", sleep_ranges[&most_asleep]);

    let big_minute = {
        let mut minutes = [0; 60];
        for range in sleep_ranges.remove(&most_asleep).unwrap() {
            for v in range {
                minutes[v as usize] += 1;
            }
        }
        let mut largest_val = 0;
        let mut largest_index = 0;
        println!("{:?}", minutes.iter().enumerate().collect::<Vec<_>>());
        (0..minutes.len()).for_each(|i| {
            if minutes[i] >= largest_val {
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
    0
}
