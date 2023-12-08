use std::collections::HashMap;

use crate::utils::load_string;

pub fn part1() -> usize {
    let binding = load_string("inputs/2023/day8.input");
    let (inst, moves) = binding.split_once("\n\n").unwrap();
    let insts = inst.chars().cycle();
    let moves = moves
        .lines()
        .map(|a| {
            let [i, a, b] = a
                .split(|c| matches!(c, ' ' | '=' | ',' | ')' | '('))
                .filter(|s| !s.is_empty())
                .array_chunks::<3>()
                .next()
                .unwrap();
            (i, (a, b))
        })
        .collect::<HashMap<_, _>>();
    let mut pos = "AAA";
    let mut i = 0;
    for inst in insts {
        let next_pos = moves[pos];
        pos = match inst {
            'L' => next_pos.0,
            'R' => next_pos.1,
            _ => unreachable!(),
        };
        i += 1;
        if pos == "ZZZ" {
            break;
        }
    }
    i
}
pub fn part2() -> usize {
    // let binding = load_string("inputs/2023/day8.input");
    // let (inst, moves) = binding.split_once("\n\n").unwrap();
    // let insts = inst.chars().cycle();
    // let moves = moves
    //     .lines()
    //     .map(|a| {
    //         let [i, a, b] = a
    //             .split(|c| matches!(c, ' ' | '=' | ',' | ')' | '('))
    //             .filter(|s| !s.is_empty())
    //             .array_chunks::<3>()
    //             .next()
    //             .unwrap();
    //         (i, (a, b))
    //     })
    //     .collect::<HashMap<_, _>>();
    // let mut poses = moves
    //     .keys()
    //     .filter(|a| a.ends_with('A'))
    //     .copied()
    //     .collect::<Vec<_>>();
    // let mut i = 0;
    // for inst in insts {
    //     for pos in poses.iter_mut() {
    //         let next_pos = moves[pos];
    //         *pos = match inst {
    //             'L' => next_pos.0,
    //             'R' => next_pos.1,
    //             _ => unreachable!(),
    //         };
    //     }
    //     i += 1;
    //     for (pi, p) in poses.iter().enumerate() {
    //         if p.ends_with('Z') {
    //             println!("{pi}: {i}");
    //         }
    //     }
    //     if poses.iter().all(|s| s.ends_with('Z')) {
    //         break;
    //     }
    // }
    // i
    // Solved using calculatr :) LCM! :)
    (1..)
        .find(|i| {
            i % 14893 == 0
                && i % 18827 == 0
                && i % 17141 == 0
                && i % 13207 == 0
                && i % 22199 == 0
                && i % 16579 == 0
        })
        .unwrap()
}
