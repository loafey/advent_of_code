pub fn part1() -> usize {
    let coords = include_str!("input/day14.input")
        .lines()
        .map(|r| {
            r.split(" -> ")
                .map(|coords| {
                    let mut splat = coords.split(',');
                    [
                        splat.next().unwrap().parse::<usize>().unwrap(),
                        splat.next().unwrap().parse::<usize>().unwrap(),
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_x = coords
        .iter()
        .flat_map(|r| r.iter().map(|[x, _]| *x))
        .max()
        .unwrap()
        + 2;
    let min_x = coords
        .iter()
        .flat_map(|r| r.iter().map(|[x, _]| *x))
        .min()
        .unwrap()
        - 2;
    let max_y = coords
        .iter()
        .flat_map(|r| r.iter().map(|[_, y]| *y))
        .max()
        .unwrap()
        + 2;
    let min_y = 0;

    let mut grid = vec![vec![' '; max_x - min_x]; max_y - min_y];
    for r in coords {
        r.windows(2).for_each(|p| {
            let [c1,c2] = p else {unreachable!()};
            if c1[0] == c2[0] {
                let bottom = c1[1].min(c2[1]);
                let top = c1[1].max(c2[1]);
                for y in bottom..=top {
                    grid[y - min_y][c1[0] - min_x] = '█';
                }
            } else {
                let bottom = c1[0].min(c2[0]);
                let top = c1[0].max(c2[0]);
                for x in bottom..=top {
                    grid[c1[1] - min_y][x - min_x] = '█';
                }
            }
        });
    }

    let start = [500 - min_x, 0];
    let mut sand = start;
    loop {
        //grid.iter().enumerate().for_each(|(y, r)| {
        //    r.iter().enumerate().for_each(|(x, c)| {
        //        if y == sand[1] && x == sand[0] {
        //            print!("ø");
        //        } else {
        //            print!("{c}");
        //        }
        //    });
        //    println!()
        //});

        let bottom = [
            grid[sand[1] + 1][sand[0] - 1] != ' ',
            grid[sand[1] + 1][sand[0]] != ' ',
            grid[sand[1] + 1][sand[0] + 1] != ' ',
        ];
        //println!();
        match bottom {
            [true, true, false] => {
                sand[0] += 1;
            }
            [false, true, false] | [false, true, true] => {
                sand[0] -= 1;
            }
            [_, true, _] => {
                grid[sand[1]][sand[0]] = '░';
                sand = start;
            }
            _ => {
                sand[1] += 1;
            }
        }
        if sand[1] >= grid.len() - 1 || sand[0] >= grid[0].len() - 1 {
            break;
        }
        //std::thread::sleep_ms(16);
    }
    grid.into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|c| *c == '░')
        .count()
}

pub fn part2() -> usize {
    let coords = include_str!("input/day14.input")
        .lines()
        .map(|r| {
            r.split(" -> ")
                .map(|coords| {
                    let mut splat = coords.split(',');
                    [
                        splat.next().unwrap().parse::<usize>().unwrap(),
                        splat.next().unwrap().parse::<usize>().unwrap(),
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_x = coords
        .iter()
        .flat_map(|r| r.iter().map(|[x, _]| *x))
        .max()
        .unwrap()
        + 300;
    let min_x = 200;
    let max_y = coords
        .iter()
        .flat_map(|r| r.iter().map(|[_, y]| *y))
        .max()
        .unwrap()
        + 3;
    let min_y = 0;

    let mut grid = vec![vec!['.'; max_x - min_x]; max_y - min_y];
    for r in coords {
        r.windows(2).for_each(|p| {
            let [c1,c2] = p else {unreachable!()};
            if c1[0] == c2[0] {
                let bottom = c1[1].min(c2[1]);
                let top = c1[1].max(c2[1]);
                for y in bottom..=top {
                    grid[y - min_y][c1[0] - min_x] = '█';
                }
            } else {
                let bottom = c1[0].min(c2[0]);
                let top = c1[0].max(c2[0]);
                for x in bottom..=top {
                    grid[c1[1] - min_y][x - min_x] = '█';
                }
            }
        });
    }
    grid[max_y - 1].iter_mut().for_each(|c| *c = '█');

    let start = [500 - min_x, 0];
    let mut sand = start;
    let mut sand_cache = sand;
    let mut still_counter = 0;
    loop {
        let bottom = [
            grid[sand[1] + 1][sand[0] - 1] != '.',
            grid[sand[1] + 1][sand[0]] != '.',
            grid[sand[1] + 1][sand[0] + 1] != '.',
        ];
        match bottom {
            [true, true, false] => {
                sand[0] += 1;
            }
            [false, true, false] | [false, true, true] => {
                sand[0] -= 1;
            }
            [_, true, _] => {
                grid[sand[1]][sand[0]] = '░';
                sand = start;
            }
            _ => {
                sand[1] += 1;
            }
        }
        if sand == sand_cache {
            still_counter += 1;
        } else {
            still_counter = 0;
        }

        if sand[1] >= grid.len() - 1 || sand[0] >= grid[0].len() - 1 || still_counter > 2 {
            break;
        }
        sand_cache = sand;
    }
    grid.into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|c| *c == '░')
        .count()
}
