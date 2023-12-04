// use chumsky::prelude::*;

// #[derive(Clone, Debug)]
// enum Instr {
//     Chars,
//     F(String),
// }

// fn parser() -> impl Parser<char, Vec<Instr>, Error = Simple<char>> {
//     recursive(|bf| {
//         choice((
//             just('c').to(Instr::Chars),
//             bf.delimited_by(just("f("), just(")"))
//                 .map(ToString::to_string)
//                 .to(Instr::F),
//         ))
//         .repeated()
//     })
// }

// // c : chars
// // f  cond : filter
// // cond "expr" : lambda

// pub fn test() {
//     println!("{:#?}", parser().parse("<>[>[--]]").unwrap())
// }
