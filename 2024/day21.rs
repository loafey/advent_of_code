use pathfinding::prelude::bfs;
use rustc_hash::FxHashMap;
use utils::{Direction, Direction::*};

fn dpad() -> FxHashMap<char, Vec<(Direction, char)>> {
    let mut map = FxHashMap::default();
    map.insert('^', vec![(Down, 'v'), (Right, 'A')]);
    map.insert('A', vec![(Down, '>'), (Left, '^')]);
    map.insert('<', vec![(Right, 'v')]);
    map.insert('v', vec![(Up, '^'), (Left, '<'), (Right, '>')]);
    map.insert('>', vec![(Up, 'A'), (Left, 'v')]);
    map
}

fn keypad() -> FxHashMap<char, Vec<(Direction, char)>> {
    let mut map = FxHashMap::default();
    map.insert('A', vec![(Up, '3'), (Left, '0')]);
    map.insert('0', vec![(Up, '2'), (Right, 'A')]);
    map.insert('1', vec![(Up, '4'), (Right, '2')]);
    map.insert('2', vec![(Up, '5'), (Down, '0'), (Left, '1'), (Right, '3')]);
    map.insert('3', vec![(Up, '6'), (Down, 'A'), (Left, '2')]);
    map.insert('4', vec![(Up, '7'), (Down, '1'), (Right, '5')]);
    map.insert('5', vec![(Up, '8'), (Down, '2'), (Left, '4'), (Right, '6')]);
    map.insert('6', vec![(Up, '9'), (Down, '3'), (Left, '5')]);
    map.insert('7', vec![(Down, '4'), (Right, '8')]);
    map.insert('8', vec![(Down, '5'), (Left, '7'), (Right, '9')]);
    map.insert('9', vec![(Down, '6'), (Left, '8')]);
    map
}

fn print_path(path: &[(Direction, char)]) {
    for (p, c) in path {
        if *c == 'P' {
            print!("A");
            continue;
        }
        match p {
            Direction::Up => print!("^"),
            Direction::Right => print!(">"),
            Direction::Down => print!("v"),
            Direction::Left => print!("<"),
        }
    }
    println!()
}

pub fn part1() -> usize {
    let mut sum = 0;
    let codes = include_str!("../inputs/2024/day21.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dpad = dpad();
    let keypad = keypad();

    // let mut posses = ['A'; 5];
    for code in codes {
        // let mut pos = posses[0];
        print!("{}: ", code.iter().collect::<String>());
        let mut nums = Vec::new();
        for keypad_goal in &code {
            if keypad_goal.is_numeric() {
                nums.push(*keypad_goal);
            }
        }
        let nums = nums.iter().collect::<String>().parse::<i64>().unwrap();
        let mut keypad_pushes = Vec::new();

        for i in 0..3 {
            let mut dpad_pushes = Vec::new();
            // let mut pos = posses[1 + i];
            let mut pos = 'A';
            for (dpad_goal, ch) in if keypad_pushes.is_empty() {
                code.iter().map(|c| (Up, *c)).collect::<Vec<_>>()
            } else {
                keypad_pushes
            } {
                let dpad_goal = if i == 0 {
                    ch
                } else if ch == 'P' {
                    'A'
                } else {
                    match dpad_goal {
                        Direction::Up => '^',
                        Direction::Right => '>',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                    }
                };
                let path = bfs(
                    &(Up, pos),
                    |(_, c)| {
                        if i == 0 { &keypad } else { &dpad }
                            .get(c)
                            .cloned()
                            .unwrap()
                    },
                    |(_, c)| *c == dpad_goal,
                )
                .unwrap();
                dpad_pushes.extend_from_slice(&path[1..]);
                dpad_pushes.push((Up, 'P'));
                pos = dpad_goal;
            }
            keypad_pushes = dpad_pushes;
        }
        print_path(&keypad_pushes);
        sum += nums as usize * keypad_pushes.len();
    }
    // println!("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
    println!(
        "
029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    );
    sum
}
// x < 213256
pub fn part2() -> i64 {
    0
}
