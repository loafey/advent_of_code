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
        .add(aoc_2018::day5::part1, aoc_2018::day5::part2)
        .add(aoc_2018::day6::part1, aoc_2018::day6::part2)
        .add(aoc_2018::day7::part1, aoc_2018::day7::part2)
        .add(aoc_2018::day8::part1, aoc_2018::day8::part2)
        .add(aoc_2018::day9::part1, aoc_2018::day9::part2)
        .add(aoc_2018::day10::part1, aoc_2018::day10::part2)
        .add(aoc_2018::day11::part1, aoc_2018::day11::part2)
        .add(aoc_2018::day12::part1, aoc_2018::day12::part2)
        .run_day(12);
}
