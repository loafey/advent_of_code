#![feature(pattern)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![allow(clippy::single_range_in_vec_init)]
#![allow(unused)]

use std::time::Duration;

mod aoc_2018;
mod aoc_2019;
mod aoc_2020;
mod aoc_2022;
mod aoc_2023;
mod chum;
mod parser;
mod utils;

fn main() {
    // chum::test();

    let benchmark = std::env::args().filter(|s| s == "--benchmark").count() == 1;
    let table = std::env::args().filter(|s| s == "--table").count() == 1;
    if benchmark {
        println!("# AoC Benchmarks");
        println!("{}\n", format_benchmark(aoc_2023::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2022::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2020::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2019::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2018::table().run_benchmarks()));
    } else if table {
        aoc_2023::table().run();
    } else {
        aoc_2023::table().run_current_day()
    }
}

#[allow(clippy::type_complexity)]
fn format_benchmark(
    (s, v): (String, Vec<(usize, (String, Duration), (String, Duration))>),
) -> String {
    let mut s = format!("## {s} \n| Day | Part 1 runtime | Part 2 runtime |\n| --- | --- | --- |");
    for (day, (a1, d1), (a2, d2)) in v {
        let r = format!(
            "\n|{}|{}|{}|",
            day,
            if !matches!(&a1[..], " " | "0" | "") {
                format_time(d1)
            } else {
                "❌".to_owned()
            },
            if !matches!(&a2[..], " " | "0" | "") {
                format_time(d2)
            } else {
                "❌".to_owned()
            },
        );
        s += &r;
    }
    s
}

fn format_time(duration: Duration) -> String {
    // if duration.as_secs_f64() >= 0.1 {
    //     format!("{}s", duration.as_secs_f64())
    // } else {
    //     format!("{}μs", duration.as_micros())
    // }
    format!("{:.8}*s*", duration.as_secs_f64())
}
