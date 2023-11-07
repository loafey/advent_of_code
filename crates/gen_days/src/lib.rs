#![feature(proc_macro_span)]

use std::fs;

use proc_macro::TokenStream;

#[proc_macro]
pub fn gen_days(item: TokenStream) -> TokenStream {
    let path = format!("{item}");
    let mut name = path.split('/').last().unwrap();
    if name.ends_with('"') {
        name = &name[..name.len() - 1];
    }
    if name.starts_with('"') {
        name = &name[1..];
    }
    let path = format!(
        "{}/../../{}",
        env!("CARGO_MANIFEST_DIR"),
        &path[1..path.len() - 1]
    );

    let files = fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|f| f.ok())
        .map(|d| d.path())
        .filter(|p| p.file_name().map(|s| s != "mod.rs").unwrap_or_default())
        .filter_map(|s| s.file_name().map(|s| s.to_string_lossy().to_string()))
        .filter_map(|s| s.split_once('.').map(|(s, _)| s.to_string()));

    let mut mods = String::new();
    let mut call = String::new();
    for f in files {
        mods += &format!("mod {f};");
        call += &format!(".add_next({f}::part1, {f}::part2)")
    }
    let call = format!(
        "pub fn table() -> aoc_table::table_gen::TableGen {{
            aoc_table::table_gen::TableGen::new(\"{name} Rust solutions 🤠\"){call}
    }}
    "
    );

    format!("{mods} {call}").parse().unwrap()
}
