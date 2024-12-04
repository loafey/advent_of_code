use std::time::Duration;

use aoc_table::table_gen::BenchmarkResults;

macro_rules! year {
    () => {
        aoc_2024::table()
    };
}

fn main() {
    let benchmark = std::env::args().filter(|s| s == "--benchmark").count() == 1;
    let table = std::env::args().filter(|s| s == "--table").count() == 1;
    let num = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok());
    if benchmark {
        println!("# AoC Benchmarks");
        println!("{}\n", format_benchmark(aoc_2024::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2023::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2022::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2020::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2019::table().run_benchmarks()));
        println!("{}\n", format_benchmark(aoc_2018::table().run_benchmarks()));
    } else if table {
        year!().run();
    } else if let Some(num) = num {
        println!("╍ Running day {num} ╍");
        year!().run_day(num)
    } else {
        println!("╍ Running current day ╍");
        year!().run_current_day()
    }
}

#[allow(clippy::type_complexity)]
fn format_benchmark((s, v): (String, Vec<BenchmarkResults>)) -> String {
    let mut s =
        format!("## {s} \n| Day | Part 1 avg | Best | Worst | Part 2 avg | Best | Worst |\n| --- | --- | --- | --- | --- | --- | --- |");
    for BenchmarkResults {
        day,
        p1_ans,
        p1_best,
        p1_worst,
        p1_avg,
        p2_ans,
        p2_best,
        p2_worst,
        p2_avg,
    } in v
    {
        let r = format!(
            "\n|{}|{}|{}|{}|{}|{}|{}|",
            day,
            if !matches!(&p1_ans[..], " " | "0" | "") {
                format!("{p1_avg:?}")
            } else {
                "❌".to_owned()
            },
            if !matches!(&p1_ans[..], " " | "0" | "") {
                format!("{p1_best:?}")
            } else {
                "❌".to_owned()
            },
            if !matches!(&p1_ans[..], " " | "0" | "") {
                format!("{p1_worst:?}")
            } else {
                "❌".to_owned()
            },
            if !matches!(&p2_ans[..], " " | "0" | "") {
                format!("{p2_avg:?}")
            } else {
                "❌".to_owned()
            },
            if !matches!(&p2_ans[..], " " | "0" | "") {
                format!("{p2_best:?}")
            } else {
                "❌".to_owned()
            },
            if !matches!(&p2_ans[..], " " | "0" | "") {
                format!("{p2_worst:?}")
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
