use crate::utils::load_string;

pub fn part1() -> usize {
    let inputs = load_string("inputs/2023/day3.input")
        .lines()
        .map(|s| {
            let mut stack = Vec::new();
            let mut word = String::new();
            for c in s.chars() {
                if c == '.' {
                    if !word.is_empty() {
                        stack.push(word);
                    }
                    stack.push(".".to_owned());
                    word = String::new();
                } else if c.is_numeric() {
                    word.push(c);
                } else {
                    if !word.is_empty() {
                        stack.push(word);
                    }
                    stack.push(c.to_string());
                    word = String::new();
                }
            }
            if !word.is_empty() {
                stack.push(word);
            }
            let mut i = 0;
            while i < stack.len() {
                if stack[i].parse::<i16>().is_ok() {
                    for l in 0..(stack[i].len() - 1) {
                        stack.insert(i + l, stack[i].clone());
                    }
                    i += stack[i].len();
                } else {
                    i += 1;
                }
            }
            stack
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for y in 0..inputs.len() {
        for x in 0..inputs[y].len() {
            if inputs[y][x].parse::<i16>().is_err() && inputs[y][x] != "." {
                // println!("{}", inputs[y][x]);
                // top
                let mut neighbors = Vec::new();
                if y > 0 {
                    neighbors.push(&inputs[y - 1][x]);
                }

                // bottom
                if y < inputs.len() {
                    neighbors.push(&inputs[y + 1][x]);
                }

                // left
                if x > 0 {
                    neighbors.push(&inputs[y][x - 1]);
                }

                // right
                if x < inputs[y].len() {
                    neighbors.push(&inputs[y][x + 1]);
                }

                // Top left
                if x > 0 && y > 0 {
                    neighbors.push(&inputs[y - 1][x - 1]);
                }

                // Top right
                if x < inputs[y].len() && y > 0 {
                    neighbors.push(&inputs[y - 1][x + 1]);
                }

                // Bottom left
                if x > 0 && y < inputs.len() {
                    neighbors.push(&inputs[y + 1][x - 1]);
                }

                // Bottom right
                if x < inputs[y].len() && y < inputs.len() {
                    neighbors.push(&inputs[y + 1][x + 1]);
                }
                let mut neighbors = neighbors
                    .into_iter()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                neighbors.sort();
                neighbors.dedup();
                // println!("{neighbors:?}");
                for n in neighbors {
                    sum += n;
                }
            }
        }
    }

    sum
}
pub fn part2() -> usize {
    let inputs = load_string("inputs/2023/day3.input")
        .lines()
        .map(|s| {
            let mut stack = Vec::new();
            let mut word = String::new();
            for c in s.chars() {
                if c == '.' {
                    if !word.is_empty() {
                        stack.push(word);
                    }
                    stack.push(".".to_owned());
                    word = String::new();
                } else if c.is_numeric() {
                    word.push(c);
                } else {
                    if !word.is_empty() {
                        stack.push(word);
                    }
                    stack.push(c.to_string());
                    word = String::new();
                }
            }
            if !word.is_empty() {
                stack.push(word);
            }
            let mut i = 0;
            while i < stack.len() {
                if stack[i].parse::<i16>().is_ok() {
                    for l in 0..(stack[i].len() - 1) {
                        stack.insert(i + l, stack[i].clone());
                    }
                    i += stack[i].len();
                } else {
                    i += 1;
                }
            }
            stack
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for y in 0..inputs.len() {
        for x in 0..inputs[y].len() {
            if inputs[y][x].parse::<i16>().is_err() && inputs[y][x] != "." {
                // println!("{}", inputs[y][x]);
                // top
                let mut neighbors = Vec::new();
                if y > 0 {
                    neighbors.push(&inputs[y - 1][x]);
                }

                // bottom
                if y < inputs.len() {
                    neighbors.push(&inputs[y + 1][x]);
                }

                // left
                if x > 0 {
                    neighbors.push(&inputs[y][x - 1]);
                }

                // right
                if x < inputs[y].len() {
                    neighbors.push(&inputs[y][x + 1]);
                }

                // Top left
                if x > 0 && y > 0 {
                    neighbors.push(&inputs[y - 1][x - 1]);
                }

                // Top right
                if x < inputs[y].len() && y > 0 {
                    neighbors.push(&inputs[y - 1][x + 1]);
                }

                // Bottom left
                if x > 0 && y < inputs.len() {
                    neighbors.push(&inputs[y + 1][x - 1]);
                }

                // Bottom right
                if x < inputs[y].len() && y < inputs.len() {
                    neighbors.push(&inputs[y + 1][x + 1]);
                }
                let mut neighbors = neighbors
                    .into_iter()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                neighbors.sort();
                neighbors.dedup();
                // println!("{neighbors:?}");
                if inputs[y][x] == "*" && neighbors.len() == 2 {
                    sum += neighbors[0] * neighbors[1];
                }
            }
        }
    }

    sum
}
