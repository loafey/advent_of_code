use std::str::FromStr;

pub fn parse_next<'l, T: FromStr>(iter: &mut impl Iterator<Item = &'l str>) -> T {
    iter.next()
        .and_then(|s| s.parse::<T>().ok())
        .expect("Failed to parse string")
}
