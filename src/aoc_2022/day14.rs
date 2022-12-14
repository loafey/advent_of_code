fn parse_inputs() -> Vec<Vec<[usize; 2]>> {
    include_str!("input/day14.input")
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
        .collect::<Vec<_>>()
}
fn create_grid(
    coords: Vec<Vec<[usize; 2]>>,
    max_x: usize,
    min_x: usize,
    max_y: usize,
) -> Vec<Vec<char>> {
    let mut grid = vec![vec![' '; max_x - min_x]; max_y];
    for r in coords {
        r.windows(2).for_each(|p| {
            let [c1,c2] = p else {unreachable!()};
            if c1[0] == c2[0] {
                let bottom = c1[1].min(c2[1]);
                let top = c1[1].max(c2[1]);
                (bottom..=top).for_each(|y| {
                    grid[y][c1[0] - min_x] = '█';
                });
            } else {
                let bottom = c1[0].min(c2[0]);
                let top = c1[0].max(c2[0]);
                for x in bottom..=top {
                    grid[c1[1]][x - min_x] = '█';
                }
            }
        });
    }
    grid
}
fn simulate(grid: &mut Vec<Vec<char>>, min_x: usize) {
    let start = [500 - min_x, 0];
    let mut sand = start;
    let mut sand_cache = sand;
    let mut still_counter = 0;
    loop {
        #[cfg(feature = "draw")]
        {
            grid.iter().enumerate().for_each(|(y, r)| {
                r.iter().enumerate().for_each(|(x, c)| {
                    if y == sand[1] && x == sand[0] {
                        print!("ø");
                    } else {
                        print!("{c}");
                    }
                });
                println!()
            });
            println!();
            std::thread::sleep_ms(16);
        }

        let bottom = [
            grid[sand[1] + 1][sand[0] - 1] != ' ',
            grid[sand[1] + 1][sand[0]] != ' ',
            grid[sand[1] + 1][sand[0] + 1] != ' ',
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
}

pub fn part1() -> usize {
    let coords = parse_inputs();

    let (max_x, min_x, max_y) = (
        coords
            .iter()
            .flat_map(|r| r.iter().map(|[x, _]| *x))
            .max()
            .unwrap()
            + 2,
        coords
            .iter()
            .flat_map(|r| r.iter().map(|[x, _]| *x))
            .min()
            .unwrap()
            - 2,
        coords
            .iter()
            .flat_map(|r| r.iter().map(|[_, y]| *y))
            .max()
            .unwrap()
            + 2,
    );

    let mut grid = create_grid(coords, max_x, min_x, max_y);
    simulate(&mut grid, min_x);

    grid.into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|c| *c == '░')
        .count()
}

pub fn part2() -> usize {
    let coords = parse_inputs();

    let (max_x, min_x, max_y) = (
        coords
            .iter()
            .flat_map(|r| r.iter().map(|[x, _]| *x))
            .max()
            .unwrap()
            + 300,
        200,
        coords
            .iter()
            .flat_map(|r| r.iter().map(|[_, y]| *y))
            .max()
            .unwrap()
            + 3,
    );

    let mut grid = create_grid(coords, max_x, min_x, max_y);
    grid[max_y - 1].iter_mut().for_each(|c| *c = '█');
    simulate(&mut grid, min_x);

    grid.into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|c| *c == '░')
        .count()
}
