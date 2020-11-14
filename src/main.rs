mod regelexer;
use regelexer_derive::LexerMacro;
use self::regelexer::lexer_enum::LexerEnumTrait;

#[derive(LexerMacro)]
enum Pancakes {
    #[regex("aaba",skip)]
    Error,
    #[token("baa")]
    Test
}

// impl LexerMacro for Pancakes {
//     fn hello_macro() {
//         println!("aa");
//     }
// }

fn main() {
}
