use crate::lexer::{Lexer, Token};

const N:usize = 5;

#[derive(Debug)]
pub struct Parser<'a>{
    lexer: &'a mut Lexer<'a>,
    token_buffer: [Option<Token<'a>>;N],
    token_cursor: usize,
}
impl<'a> Parser<'a>{
    pub fn new(lexer: &'a mut Lexer<'a>)->Self{
        let token_buffer = std::array::from_fn(|_| lexer.next());

        Parser { lexer, token_buffer, token_cursor: 0 }
    }
}
