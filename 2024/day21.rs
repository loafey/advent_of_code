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
        let mut pos = 'A';
        print!("{}: ", code.iter().collect::<String>());
        let mut keypad_pushes = Vec::new();
        let mut nums = Vec::new();
        for keypad_goal in code {
            if keypad_goal.is_numeric() {
                nums.push(keypad_goal);
            }
            let path = bfs(
                &(Up, pos),
                |(_, c)| keypad.get(c).cloned().unwrap_or_default(),
                |(_, c)| *c == keypad_goal,
            )
            .unwrap();
            keypad_pushes.extend_from_slice(&path[1..]);
            keypad_pushes.push((Up, 'P'));
            pos = keypad_goal;
        }
        let nums = nums.iter().collect::<String>().parse::<i64>().unwrap();
        // posses[0] = pos;
        // println!("{keypad_pushes:?}");
        // print_path(&keypad_pushes);
        // println!("    <A^A>^^AvvvA");

        // let arr = [
        //     "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
        //     "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
        // ];

        for i in 0..2 {
            let mut dpad_pushes = Vec::new();
            // let mut pos = posses[1 + i];
            let mut pos = 'A';
            for (dpad_goal, ch) in keypad_pushes {
                let dpad_goal = if ch == 'P' {
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
                    |(_, c)| dpad.get(c).cloned().unwrap(),
                    |(_, c)| *c == dpad_goal,
                )
                .unwrap();
                dpad_pushes.extend_from_slice(&path[1..]);
                dpad_pushes.push((Up, 'P'));
                pos = dpad_goal;
            }
            // posses[i + 1] = pos;
            keypad_pushes = dpad_pushes;
            for i in 0..keypad_pushes.len() - 2 {
                let a = keypad_pushes[i];
                let b = keypad_pushes[i + 1];
                let c = keypad_pushes[i + 2];
                if a.1 == 'P' || b.1 == 'P' || c.1 == 'P' {
                    continue;
                }
                // print!("{:?} ", [a.0, b.0, c.0]);
                match [a.0, b.0, c.0] {
                    [Right, Up, Right]
                    | [Right, Down, Right]
                    | [Left, Up, Left]
                    | [Left, Down, Left]
                    | [Up, Left, Up]
                    | [Up, Right, Up]
                    | [Down, Left, Down]
                    | [Down, Right, Down] => {
                        keypad_pushes[i] = a;
                        keypad_pushes[i + 1] = c;
                        keypad_pushes[i + 2] = b;
                    }
                    _ => {}
                }
            }
            // println!();
            // print_path(&keypad_pushes);
            // println!("    {}", arr[i]);
        }
        print_path(&keypad_pushes);
        // println!(
        //     "{nums} * {} = {}",
        //     keypad_pushes.len(),
        //     nums as usize * keypad_pushes.len()
        // );
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
