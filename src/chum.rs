use chumsky::prelude::*;

#[derive(Clone, Debug)]
enum Instr<'a> {
    Chars,
    F(&'a str),
}

fn parser<'a>() -> impl Parser<&'a str, Vec<Instr<'a>>> {
    recursive(|bf| {
        choice((
            just("c").to(Instr::Chars),
            bf.delimited_by(just("f("), just(")")).map(Instr::F),
        ))
        .repeated()
    })
}

// c : chars
// f  cond : filter
// cond "expr" : lambda

pub fn test() {
    println!("{:#?}", parser().parse("<>[>[--]]").unwrap())
}
