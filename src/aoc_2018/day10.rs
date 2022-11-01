use crate::utils::parse_next;

#[derive(Debug, Clone, Copy)]
struct Line {
    pos: (i32, i32),
    vec: (i32, i32),
}
impl Line {
    pub fn tick(&mut self) {
        self.pos.0 += self.vec.0;
        self.pos.1 += self.vec.1;
    }
}

#[allow(unused_variables)]
pub fn part1() -> &'static str {
    let mut lights = include_str!("input/day10.input")
        .split('\n')
        .map(|r| {
            let mut split = r
                .split(|c| ['<', '>', ','].contains(&c))
                .skip(1)
                .map(|s| s.trim());
            let pos_x = parse_next::<i32>(&mut split);
            let pos_y = parse_next::<i32>(&mut split);
            split.next();
            let vec_x = parse_next::<i32>(&mut split);
            let vec_y = parse_next::<i32>(&mut split);

            Line {
                pos: (pos_x, pos_y),
                vec: (vec_x, vec_y),
            }
        })
        .collect::<Vec<_>>();

    let mut second = 0;
    loop {
        const X_MAX: usize = 150;
        const Y_MAX: usize = 10;
        let mut arr = [['.'; X_MAX]; Y_MAX];
        let min_pos_x = lights
            .iter()
            .min_by_key(|l| l.pos.0)
            .map(|l| l.pos.0)
            .unwrap_or_default();
        let min_pos_y = lights
            .iter()
            .min_by_key(|l| l.pos.1)
            .map(|l| l.pos.1)
            .unwrap_or_default();

        let mut test_count = 0;
        for l in lights.iter_mut() {
            if l.pos.0 - min_pos_x >= 0
                && l.pos.0 - min_pos_x < X_MAX as i32
                && l.pos.1 - min_pos_y >= 0
                && l.pos.1 - min_pos_y < Y_MAX as i32
            {
                arr[(l.pos.1 - min_pos_y) as usize][(l.pos.0 - min_pos_x) as usize] = '#';
                test_count += 1;
            }

            l.tick();
        }

        if test_count == lights.len() {
            // println!("Second {second}: {min_pos_y}");
            // arr.into_iter().for_each(|r| {
            //     r.into_iter().for_each(|c| print!("{c}"));
            //     println!()
            // });
            break;
        }
        second += 1;
    }

    "XPFXXXKL (uncomment print statement)"
}

pub fn part2() -> &'static str {
    "10521 (see left comment)"
}
