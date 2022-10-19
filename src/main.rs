use aoc_table::table_gen::TableGen;
mod aoc_2018;

fn main() {
    //AOC_2018::day1::part2();
    TableGen::new("2018 solutions")
        .add(aoc_2018::day1::part1, aoc_2018::day1::part2)
        .run();
}
