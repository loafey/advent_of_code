#![feature(pattern)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]
//mod aoc_2018;
//mod aoc_2022;
mod aoc_2019;
mod aoc_2020;
mod aoc_2023;
mod parser;
mod utils;

fn main() {
    aoc_2020::table().run_day(3);
}
