use utils::load_string;
use std::collections::BTreeMap as Map;

pub fn part1() -> usize {
    load_string("inputs/2020/day4.input")
        .split("\n\n")
        .map(|p| {
            p.split([' ', '\n'])
                .filter_map(|s| s.split_once(':'))
                .collect::<Map<_, _>>()
        })
        .filter(|m| m.len() == 8 || m.len() == 7 && !m.contains_key("cid"))
        .count()
}

fn validate(val: &str, bot: usize, top: usize) -> Option<()> {
    (bot..=top)
        .contains(&val.parse::<usize>().ok()?)
        .then_some(())
}

pub fn part2() -> usize {
    load_string("inputs/2020/day4.input")
        .split("\n\n")
        .map(|p| {
            p.split([' ', '\n'])
                .filter_map(|s| s.split_once(':'))
                .collect::<Map<_, _>>()
        })
        .filter_map(|m| try {
            validate(m.get("byr")?, 1920, 2002)?;
            validate(m.get("iyr")?, 2010, 2020)?;
            validate(m.get("eyr")?, 2020, 2030)?;
            let hgt = m.get("hgt")?;
            if hgt.ends_with("in") {
                validate(&hgt[0..hgt.len() - 2], 59, 76)?;
            } else if hgt.ends_with("cm") {
                validate(&hgt[0..hgt.len() - 2], 150, 193)?;
            } else {
                None?;
            }
            let hcl = m.get("hcl")?;
            (hcl.starts_with('#')
                && hcl[1..].len() == 6
                && hcl[1..]
                    .chars()
                    .all(|c| c.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c)))
            .then_some(())?;

            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .contains(m.get("ecl")?)
                .then_some(())?;
            let pid = m.get("pid")?;
            (pid.len() == 9).then_some(())?;
            pid.parse::<usize>().ok()?;
        })
        .count()
}
