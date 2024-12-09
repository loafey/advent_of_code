use std::time::Duration;

use aoc_table::table_gen::BenchmarkResults;

macro_rules! year {
    () => {
        aoc_2024::table()
    };
}

fn main() {
    {
        // Trick rayon to spin up worker threads before running days
        use rayon::prelude::*;
        (0..1).into_par_iter().sum::<i64>();
    }

    let benchmark = std::env::args().filter(|s| s == "--benchmark").count() == 1;
    let table = std::env::args().filter(|s| s == "--table").count() == 1;
    let num = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok());
    if benchmark {
        println!("# AoC Benchmarks");
        println!(
            "{}\n",
            format_benchmark(aoc_2024::table().run_benchmarks(100, true))
        );
        // println!("{}\n", format_benchmark(aoc_2023::table().run_benchmarks()));
        // println!("{}\n", format_benchmark(aoc_2022::table().run_benchmarks()));
        // println!("{}\n", format_benchmark(aoc_2020::table().run_benchmarks()));
        // println!("{}\n", format_benchmark(aoc_2019::table().run_benchmarks()));
        // println!("{}\n", format_benchmark(aoc_2018::table().run_benchmarks()));
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

fn format_time(time: Duration) -> String {
    let time = if time.as_secs() > 1 {
        format!("{}s", time.as_secs())
    } else if time.as_millis() > 1 {
        format!("{}ms", time.as_millis())
    } else if time.as_micros() > 1 {
        format!("{}µs", time.as_micros())
    } else {
        format!("{}ns", time.as_nanos())
    };
    let color = if time.contains("µs") {
        "🦀"
    } else if time.contains("ms") {
        "💅"
    } else {
        "🤡"
    };
    format!("{color}: {time}")
}

#[allow(clippy::type_complexity)]
fn format_benchmark((s, v): (String, Vec<BenchmarkResults>)) -> String {
    let mut s =
        format!("## {s} \n| Day | Part 1 avg | Best | Worst | Part 2 avg | Best | Worst |\n| --- | --- | --- | --- | --- | --- | --- |");

    let mut total_time_avg = Duration::default();
    let mut total_time_best = Duration::default();
    let mut total_time_worst = Duration::default();
    let total_amount = v.len();
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
        total_time_best += p1_best + p2_best;
        total_time_avg += p1_avg + p2_avg;
        total_time_worst += p1_worst + p2_worst;
        let r = format!(
            "\n|{}|{}|{}|{}|{}|{}|{}|",
            day,
            if !matches!(&p1_ans[..], " " | "0" | "") {
                format_time(p1_avg)
            } else {
                "❌".to_owned()
            },
            if !matches!(&p1_ans[..], " " | "0" | "") {
                format_time(p1_best)
            } else {
                "❌".to_owned()
            },
            if !matches!(&p1_ans[..], " " | "0" | "") {
                format_time(p1_worst)
            } else {
                "❌".to_owned()
            },
            if !matches!(&p2_ans[..], " " | "0" | "") {
                format_time(p2_avg)
            } else {
                "❌".to_owned()
            },
            if !matches!(&p2_ans[..], " " | "0" | "") {
                format_time(p2_best)
            } else {
                "❌".to_owned()
            },
            if !matches!(&p2_ans[..], " " | "0" | "") {
                format_time(p2_worst)
            } else {
                "❌".to_owned()
            },
        );
        s += &r;
    }
    s += "\n\n| Total average time | Total best time | Total worst time |\n| --- | --- | --- |";
    s += &format!("\n| {total_time_avg:?} | {total_time_best:?} | {total_time_worst:?} |");
    s += "\n\n| Average average time | Average best time | Average worst time |\n| --- | --- | --- |";
    s += &format!(
        "\n| {:?} | {:?} | {:?} |",
        total_time_avg / total_amount as u32,
        total_time_best / total_amount as u32,
        total_time_worst / total_amount as u32
    );
    s
}
