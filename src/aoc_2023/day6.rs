use crate::utils::{load_string, BiFunctor, BiFunctorExt, BiFunctorExtExt};

type Time = usize;
type Distance = usize;
fn inputs() -> impl Iterator<Item = (Time, Distance)> {
    let s = load_string("inputs/2023/day6.input");
    let (t, d) = s.split_once('\n').unwrap();
    let times = t
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distance = d
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    times.into_iter().zip(distance)
}

fn calc_winners(time: Time, distance: Distance) -> usize {
    let mut winners = 0;
    for hold_time in 0..=time {
        let speed = hold_time;
        let attempt = (time - hold_time) * speed;
        if attempt > distance {
            winners += 1;
        }
    }
    winners
}

pub fn part1() -> usize {
    inputs()
        .map(|(time, distance)| calc_winners(time, distance))
        .product()
}

pub fn part2() -> usize {
    inputs()
        .fold(
            (String::new(), String::new()),
            |(ts, ds), (time, distance)| (format!("{ts}{time}"), format!("{ds}{distance}")),
        )
        .splet(|s| s.parse::<usize>().unwrap())
        .splot(calc_winners)
}
