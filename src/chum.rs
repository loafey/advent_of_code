use chumsky::prelude::*;

#[derive(Clone, Debug)]
enum Instr {
    Chars,
    Space,
    Block(String),
}

fn parser() -> impl Parser<char, Vec<Instr>, Error = Simple<char>> {
    let s = text::ident().map(ToString::to_string).padded();
    recursive(|bf| {
        choice((
            just(' ').to(Instr::Space),
            just('C').to(Instr::Chars),
            bf.delimited_by(just("f("), just(')')),
        ))
        .repeated()
    })
}

// c : chars
// f  cond : filter
// cond "expr" : lambda

pub fn test() {
    println!("{:#?}", parser().parse("C f(C)").unwrap())
}
