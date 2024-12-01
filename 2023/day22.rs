use std::collections::HashSet;

use euclid::{Box3D, Point3D};
use rayon::iter::{ParallelBridge, ParallelIterator};

use utils::load_string;

fn parse_vec3(s: &str) -> Point3D<usize, ()> {
    let [x, y, z] = s
        .split(',')
        .map(|n| n.parse().unwrap())
        .array_chunks::<3>()
        .next()
        .unwrap();
    Point3D::new(x, y, z)
}

type Shape = Box3D<usize, ()>;

fn simulate(shapes: &mut [Shape]) -> HashSet<usize> {
    let org = shapes.to_owned();
    let mut modified = HashSet::new();
    for (i, shape) in shapes.iter_mut().enumerate() {
        let mut c = *shape;
        if c.max.z == 0 || c.min.z == 0 {
            continue;
        }
        c.min.z -= 1;
        c.max.z -= 1;
        let mut collided = false;
        for other_shape in &org {
            if shape == other_shape {
                continue;
            }
            if other_shape.intersects(&c) {
                collided = true;
                break;
            }
        }
        if !collided {
            *shape = c;
            modified.insert(i);
        }
    }
    modified
}

pub fn part1() -> usize {
    let mut shapes = load_string("inputs/2023/day22.input")
        .lines()
        .map(|s| {
            let (l, r) = s.split_once('~').unwrap();
            let min = parse_vec3(l);
            let mut max = parse_vec3(r);
            max.x += 1;
            max.y += 1;
            max.z += 1;
            Shape { min, max }
        })
        .collect::<Vec<_>>();

    while !simulate(&mut shapes).is_empty() {}
    let mut ans = 0;
    for (i, _) in shapes.iter().enumerate() {
        let mut clone = shapes.clone();
        clone.remove(i);
        if simulate(&mut clone).is_empty() {
            ans += 1;
            // println!("can remove {}", (i + 65) as u8 as char);
        }
    }
    ans
}

pub fn part2() -> usize {
    let mut shapes = load_string("inputs/2023/day22.input")
        .lines()
        .map(|s| {
            let (l, r) = s.split_once('~').unwrap();
            let min = parse_vec3(l);
            let mut max = parse_vec3(r);
            max.x += 1;
            max.y += 1;
            max.z += 1;
            Shape { min, max }
        })
        .collect::<Vec<_>>();

    while !simulate(&mut shapes).is_empty() {}
    shapes
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(i, _)| {
            let mut clone = shapes.clone();
            clone.remove(i);
            let mut removed = HashSet::new();
            loop {
                let removed_new = simulate(&mut clone);
                if removed_new.is_empty() {
                    break;
                }
                removed_new.into_iter().for_each(|i| {
                    removed.insert(i);
                });
            }
            removed.len()
        })
        .sum()
}
