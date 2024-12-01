use utils::{load_string, IteratorEvalExt as _};
use euclid::default::{Point2D, Point3D, Vector2D, Vector3D};
use rayon::iter::{ParallelBridge, ParallelIterator as _};
use std::{collections::HashSet, fmt::Debug};

fn parse_p3(s: &str) -> [f64; 3] {
    s.split(", ")
        .map(|p| p.trim().parse::<f64>().unwrap())
        .array_chunks::<3>()
        .next()
        .unwrap()
}
#[derive(Debug, PartialEq, Clone, Copy)]
struct Hailstone {
    pos: Point3D<f64>,
    vec: Vector3D<f64>,
}

// Ugly fix for the unhashable nature of f64 :)
struct HashHailstone((Hailstone, Hailstone));
impl Eq for HashHailstone {}
impl PartialEq for HashHailstone {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self.0) == format!("{:?}", other.0)
    }
}
impl std::hash::Hash for HashHailstone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{:?}", self.0).hash(state);
    }
}

fn input() -> Vec<Hailstone> {
    load_string("inputs/2023/day24.input")
        .lines()
        .map(|r| {
            let (p, v) = r.split_once(" @ ").unwrap();
            let p = parse_p3(p);
            let v = parse_p3(v);
            Hailstone {
                pos: Point3D::from(p),
                vec: Vector3D::from(v),
            }
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq)]
struct Line {
    d: f64,
    m: f64,
}
impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y = {}x + {}", self.d, self.m)
    }
}
impl From<(Point2D<f64>, Vector2D<f64>)> for Line {
    fn from((p, v): (Point2D<f64>, Vector2D<f64>)) -> Self {
        let p2 = p + v;
        let d = (p.y - p2.y) / (p.x - p2.x);
        let m = p.y - (p.x * d);
        Line { d, m }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct GeneralForm {
    x: f64,
    y: f64,
    z: f64,
}
impl std::fmt::Debug for GeneralForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x + {}y + {} = 0", self.x, self.y, self.z)
    }
}
impl From<Line> for GeneralForm {
    fn from(value: Line) -> Self {
        let Line { d, m } = value;

        Self {
            x: -d,
            y: 1.0,
            z: -m,
        }
    }
}

pub fn part1() -> usize {
    let stones = input();

    let mut done_combos = HashSet::new();

    //const MIN: f64 = 7.0;
    //const MAX: f64 = 27.0;
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;

    let mut ok = 0;
    // let mut i = 1;
    for a in &stones {
        for b in &stones {
            if a == b {
                continue;
            }
            if done_combos.contains(&HashHailstone((*a, *b)))
                || done_combos.contains(&HashHailstone((*b, *a)))
            {
                continue;
            }
            done_combos.insert(HashHailstone((*a, *b)));

            let pa = a.pos.xy();
            let va = a.vec.xy();
            let ga = GeneralForm::from(Line::from((a.pos.xy(), a.vec.xy())));

            let pb = b.pos.xy();
            let vb = b.vec.xy();
            let lb = Line::from((pb, vb));
            let gb = GeneralForm::from(lb);

            // println!("-- {i} -- ");
            // println!("A: {:?} @ {:?}", pa, va);
            // println!("b: {:?} @ {:?}", pb, vb);
            let intersection = Point2D::new(
                (ga.y * gb.z - gb.y * ga.z) / (ga.x * gb.y - gb.x * ga.y),
                (ga.z * gb.x - gb.z * ga.x) / (ga.x * gb.y - gb.x * ga.y),
            );
            let valid = (intersection.x >= MIN && intersection.x <= MAX)
                && (intersection.y >= MIN && intersection.y <= MAX)
                && if va.x < 0.0 {
                    intersection.x <= pa.x
                } else {
                    intersection.x >= pa.x
                }
                && if vb.x < 0.0 {
                    intersection.x <= pb.x
                } else {
                    intersection.x >= pb.x
                };
            if valid {
                ok += 1;
                // println!("         \x1b[32mOk collision\x1b[0m: {intersection:?}");
            } //else {
              // println!("    \x1b[31mInvalid collision\x1b[0m: {intersection:?}")
              // }
              // println!();

            // i += 1;
        }
    }
    ok
}

fn solver(
    mut p: Point3D<f64>,
    v: Vector3D<f64>,
    mut stones: Vec<(Point3D<f64>, Vector3D<f64>, f64)>,
) -> Option<f64> {
    while !stones.is_empty() {
        // println!("{org_p:?} {v:?}");
        p += v;
        stones.iter_mut().for_each(|(p, v, _)| *p += *v);

        let mut offset = 0;
        stones
            .iter()
            .enumerate()
            .filter(|(_, (p2, _, _))| *p2 == p)
            .map(|(i, _)| i)
            .eval()
            .for_each(|i| {
                stones.remove(i - offset);
                offset += 1;
            });

        if stones.is_empty() {
            return Some(p.x + p.y + p.z);
        }

        if stones.iter().any(|(a, _, l)| a.distance_to(p) > *l) {
            break;
        }
        stones
            .iter_mut()
            .for_each(|(a, _, l)| *l = a.distance_to(p));
    }
    None
}

pub fn part2() -> i64 {
    let stones = input()
        .into_iter()
        .map(|hs| (hs.pos, hs.vec, f64::INFINITY))
        .collect::<Vec<_>>();

    stones
        .iter()
        .enumerate()
        .skip(15)
        .par_bridge()
        .map(|(i, stone)| {
            println!("Working on stone \x1b[32m{i}\x1b[0m");
            let max_x = stone.1.x.abs() as i64 * 4;
            let max_y = stone.1.y.abs() as i64 * 4;
            let max_z = stone.1.z.abs() as i64 * 4;
            for z in (-max_z..=max_z).map(|z| z as f64) {
                for y in (-max_y..=max_y).map(|y| y as f64) {
                    for x in (-max_x..=max_x).map(|x| x as f64) {
                        let mut p = stone.0;
                        p.x += x;
                        p.y += y;
                        p.z += z;
                        let goal = stone.0 + stone.1;
                        let v = goal - p;
                        if solver(p, v, stones.clone()).is_some() {
                            return Some(p);
                        }
                    }
                }
            }
            None
        })
        .find_first(|x| x.is_some())
        .map(|p| {
            let p = p.unwrap();
            (p.x + p.y + p.z) as i64
        })
        .unwrap()
}

// if let Some(res) = solver((x, y, z), p, v, stones.clone()) {
// print!("{p:?} | {v:?} = ");
// println!("{res:?}",);
// }
