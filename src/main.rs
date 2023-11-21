#![feature(pattern)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![allow(clippy::single_range_in_vec_init)]

mod aoc_2018;
mod aoc_2019;
mod aoc_2020;
mod aoc_2022;
mod aoc_2023;
mod parser;
mod utils;

fn main() {
    aoc_2018::table().run();
    aoc_2019::table().run();
    aoc_2020::table().run();
    aoc_2022::table().run();
    aoc_2023::table().run();
}
