#![feature(proc_macro_span)]
extern crate proc_macro;
use std::path::PathBuf;

use proc_macro::{Span, TokenStream};

#[proc_macro]
pub fn matrixy(item: TokenStream) -> TokenStream {
    let mut source_path = Span::call_site().source_file().path();
    source_path.pop();
    let f = format!("{item}");
    let file_path = PathBuf::from(&f[1..(f.len() - 1)]);
    source_path = source_path.join(file_path);
    let s = std::fs::read_to_string(source_path).expect("failed to find file");
    let end = s
        .find("\n")
        .expect("could not find any new lines, is this a matrix?");
    s.lines().for_each(|s| {
        if s.len() != end {
            panic!("uneven input")
        }
    });
    let lines = s.lines().filter(|s| !s.is_empty()).count();

    format!(
        "
type Map = &'static [[u8; {end} + 1]; {lines}]; 
static MAP: Map = unsafe {{ std::mem::transmute::<&'static str, (Map, usize)>({s:?}).0 }};
"
    )
    .parse()
    .expect("outputted invalid Rust code")
}
