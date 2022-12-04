use std::{fmt::Debug, str::FromStr};

#[allow(unused)]
pub fn parse_next<'l, T: FromStr>(iter: &mut impl Iterator<Item = &'l str>) -> T {
    let s = iter.next();
    s.and_then(|s| s.parse::<T>().ok())
        .unwrap_or_else(|| panic!("Failed to parse string: \"{s:?}\""))
}

pub fn parse<F: FromStr + Debug>(s: &str) -> F {
    s.parse::<F>().ok().unwrap()
}
