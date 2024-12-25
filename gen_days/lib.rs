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
        "{}/../{}",
        env!("CARGO_MANIFEST_DIR"),
        &path[1..path.len() - 1]
    );

    let mut files = fs::read_dir(&path)
        .unwrap()
        .filter_map(|f| f.ok())
        .map(|d| d.path())
        .filter(|p| {
            p.file_name()
                .map(|s| s != "mod.rs" && s != "lib.rs" && s != "Cargo.toml")
                .unwrap_or_default()
                && p.extension().map(|s| s == "rs").unwrap_or_default()
        })
        .filter_map(|s| s.file_name().map(|s| s.to_string_lossy().to_string()))
        .filter(|s| !s.ends_with(".input"))
        .filter_map(|s| s.split_once('.').map(|(s, _)| s.to_string()))
        .collect::<Vec<_>>();
    files.sort_by_key(|s1| string_to_int(s1));

    let mut mods = String::new();
    let mut call = String::new();
    for f in &files {
        mods += &format!("mod {f};");
        call += &format!(".add_next({f}::part1, {f}::part2)")
    }
    let call = format!(
        "pub fn table() -> aoc_table::table_gen::TableGen {{
            aoc_table::table_gen::TableGen::new(\"{name} Rust solutions 🤠\"){call}
    }}"
    );

    format!("{mods} {call}").parse().unwrap()
}

fn string_to_int(s: &str) -> i32 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}
