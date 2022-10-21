use aoc_table::table_gen::TableGen;
mod aoc_2018;
mod utils;

fn main() {
    // dbg!(aoc_2018::day4::part2());
    TableGen::new("2018 solutions")
        .add(aoc_2018::day1::part1, aoc_2018::day1::part2)
        .add(aoc_2018::day2::part1, aoc_2018::day2::part2)
        .add(aoc_2018::day3::part1, aoc_2018::day3::part2)
        .add(aoc_2018::day4::part1, aoc_2018::day4::part2)
        .run();
}
