use aoc_table::table_gen::TableGen;
use utils::Zipper2D;
//mod aoc_2018;
mod aoc_2022;
mod utils;

fn main() {
    // dbg!(aoc_2018::day4::part2());
    // TableGen::new("2018 solutions")
    //     .add(aoc_2018::day1::part1, aoc_2018::day1::part2)
    //     .add(aoc_2018::day2::part1, aoc_2018::day2::part2)
    //     .add(aoc_2018::day3::part1, aoc_2018::day3::part2)
    //     .add(aoc_2018::day4::part1, aoc_2018::day4::part2)
    //     .add(aoc_2018::day5::part1, aoc_2018::day5::part2)
    //     .add(aoc_2018::day6::part1, aoc_2018::day6::part2)
    //     .add(aoc_2018::day7::part1, aoc_2018::day7::part2)
    //     .add(aoc_2018::day8::part1, aoc_2018::day8::part2)
    //     .add(aoc_2018::day9::part1, aoc_2018::day9::part2)
    //     .add(aoc_2018::day10::part1, aoc_2018::day10::part2)
    //     .add(aoc_2018::day11::part1, aoc_2018::day11::part2)
    //     .add(aoc_2018::day12::part1, aoc_2018::day12::part2)
    //     .run();

    TableGen::new("22' Rust AoC solutions 🤠")
        .add(aoc_2022::day1::part1, aoc_2022::day1::part2)
        .add(aoc_2022::day2::part1, aoc_2022::day2::part2)
        .add(aoc_2022::day3::part1, aoc_2022::day3::part2)
        .add(aoc_2022::day4::part1, aoc_2022::day4::part2)
        .add(aoc_2022::day5::part1, aoc_2022::day5::part2)
        .add(aoc_2022::day6::part1, aoc_2022::day6::part2)
        .add(aoc_2022::day7::part1, aoc_2022::day7::part2)
        .add(aoc_2022::day8::part1, aoc_2022::day8::part2)
        .add(aoc_2022::day9::part1, aoc_2022::day9::part2)
        .add(aoc_2022::day10::part1, aoc_2022::day10::part2)
        .add(aoc_2022::day11::part1, aoc_2022::day11::part2)
        .add(aoc_2022::day12::part1, aoc_2022::day12::part2)
        .add(aoc_2022::day13::part1, aoc_2022::day13::part2)
        .add(aoc_2022::day14::part1, aoc_2022::day14::part2)
        .add(aoc_2022::day15::part1, aoc_2022::day15::part2)
        .add(aoc_2022::day16::part1, aoc_2022::day16::part2)
        .add(aoc_2022::day17::part1, aoc_2022::day17::part2)
        .add(aoc_2022::day18::part1, aoc_2022::day18::part2)
        .add(aoc_2022::day19::part1, aoc_2022::day19::part2)
        .add(aoc_2022::day20::part1, aoc_2022::day20::part2)
        .add(aoc_2022::day21::part1, aoc_2022::day21::part2)
        .add(aoc_2022::day22::part1, aoc_2022::day22::part2)
        .add(aoc_2022::day23::part1, aoc_2022::day23::part2)
        .add(aoc_2022::day24::part1, aoc_2022::day24::part2)
        .add(aoc_2022::day25::part1, aoc_2022::day25::part2)
        .run();
}
