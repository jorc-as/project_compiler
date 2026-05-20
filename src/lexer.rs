#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
}
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType<'a> {
    /* ( */
    LeftParenthesis,

    /* ) */
    RightParenthesis,

    /* [ */
    LeftBraces,

    /* ] */
    RightBraces,

    /* { */
    LeftBrackets,

    /* } */
    RightBrackets,

    /* . */
    Point,

    /* + */
    Plus,

    /* - */
    Minus,

    /* / */
    Slash,

    /* * */
    Star,

    /* , */
    Comma,

    /* : */
    Colon,

    /* | */
    Pipe,

    /* ! */
    Exclamation,

    /* = */
    Equals,

    /* == */
    Assign,

    /* < */
    Less,

    /* > */
    Greater,

    /* <= */
    GreaterEq,

    /* >= */
    LessEq,

    /* % */
    Percent,

    /* ++ */
    Increment,

    /* -- */
    Decrement,

    /* != */
    NotEqual,

    /* << */
    LeftShift,

    /* >> */
    RightShift,

    /* & */
    Ampersand,

    /* ^ */
    Caret,

    /*   */
    Space,

    /* \n */
    EOF,

    /* 67 */
    Literal{
        value: &'a str,
        is_int: bool,
    },

    /* foo */
    Identifier{
        name: &'a str,
    },

    /* fn */
    KeyWord(KeyWord),
}
#[derive(Debug,PartialEq,Clone)]
pub enum KeyWord {
   For,
   If,
   Else,
   Fn,
   Print,
   Input,
   Return,
}
#[derive(Debug,Clone)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
}
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer { source, index: 0 }
    }
    /// Looks at the next character without consuming it
    fn peek(&self) -> Option<char> {
        return self.source[self.index..].chars().next();
    }
    /// Looks at the second next character without consuming it
    fn peek2(&self) -> Option<char> {
        return self.source[self.index..].chars().nth(1);
    }
    /// Looks at the next character and consumes it
    fn next_char(&mut self) -> Option<char> {
        let nxt_char = self.source[self.index..].chars().next();
        if nxt_char.is_some(){
            self.index += nxt_char.unwrap().len_utf8();
        }
        return nxt_char;
    }
    /// Gets the source code scanned directly
    pub fn scan_src(&'a mut self) -> Vec<Token<'a>> {
        let mut tokens: Vec<Token> = Vec::new();
        for token in self.into_iter() {
            // We don't want the blank spaces, we only care about EOF
            if token.token_type != TokenType::Space {
                println!(
                    "Token type: {:?}",
                    token.token_type
                );
                tokens.push(token);
            }
        }
        return tokens;
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_char() {
            Some('=') => match self.peek() {
                Some('=') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Equals,
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Assign,
                    });
                }
            },
            Some('+') => match self.peek() {
                Some('+') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Increment,
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Plus,
                    });
                }
            },
            Some('-') => match self.peek() {
                Some('-') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Decrement,
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Minus,
                    });
                }
            },
            Some('*') => {
                return Some(Token {
                    token_type: TokenType::Star,
                });
            }
            Some('/') => {
                return Some(Token {
                    token_type: TokenType::Slash,
                });
            }
            Some('(') => {
                return Some(Token {
                    token_type: TokenType::LeftParenthesis,
                });
            }
            Some(')') => {
                return Some(Token {
                    token_type: TokenType::RightParenthesis,
                });
            }
            Some('{') => {
                return Some(Token {
                    token_type: TokenType::LeftBraces,
                });
            }
            Some('}') => {
                return Some(Token {
                    token_type: TokenType::RightBraces,
                });
            }
            Some('[') => {
                return Some(Token {
                    token_type: TokenType::LeftBrackets,
                });
            }
            Some(']') => {
                return Some(Token {
                    token_type: TokenType::RightBrackets,
                });
            }
            Some('.') => {
                return Some(Token {
                    token_type: TokenType::Point,
                });
            }
            Some(':') => {
                return Some(Token {
                    token_type: TokenType::Colon,
                });
            }
            Some(',') => {
                return Some(Token {
                    token_type: TokenType::Comma,
                });
            }
            Some('|') => {
                return Some(Token {
                    token_type: TokenType::Pipe,
                });
            }
            Some('!') => match self.peek() {
                Some('=') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::NotEqual,
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Exclamation,
                    });
                }
            },
            Some('<') => match self.peek() {
                Some('=') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::LessEq,
                    });
                }
                Some('<') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::LeftShift,
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Less,
                    });
                }
            },
            Some('>') => match self.peek() {
                Some('=') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::GreaterEq,
                    });
                }
                Some('>') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::RightShift,
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Greater,
                    });
                }
            },
            Some('%') => {
                return Some(Token {
                    token_type: TokenType::Percent,
                });
            }
            Some('&') => {
                return Some(Token {
                    token_type: TokenType::Ampersand,
                });
            }
            Some('^') => {
                return Some(Token {
                    token_type: TokenType::Caret,
                });
            }
            Some(' ') => return self.next(), 
            //{
            //    return Some(Token {
            //        token_type: TokenType::Space,
            //        token_str: " ",
            //    });
            //}
            Some('\n') => {
                return Some(Token {
                    token_type: TokenType::EOF,
                });
            }
            Some(d @ '0'..='9') => {
                let start = self.index - d.len_utf8();
                if d == '0' {
                    match self.peek() {
                        Some('.') => {
                            self.next_char();
                            loop {
                                match self.peek() {
                                    Some('0'..='9') => {
                                        self.next_char();
                                    }
                                    _ => {
                                        return Some(Token {
                                            token_type: TokenType::Literal{
                                                value: &self.source[start..self.index],
                                                is_int: false,
                                            }
                                        });
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                } else {
                    loop {
                        match self.peek() {
                            Some('0'..='9') => {
                                self.next_char();
                            }
                            Some('.') => match self.peek2() {
                                Some('0'..='9') => {
                                    self.next_char();
                                    self.next_char();
                                    loop {
                                        match self.peek() {
                                            Some('0'..='9') => {
                                                self.next_char();
                                            }
                                            _ => {
                                                return Some(Token {
                                                    token_type: TokenType::Literal{
                                                        value: &self.source[start..self.index],
                                                        is_int: false,
                                                    }
                                                });
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    return Some(Token {
                                        token_type: TokenType::Literal{
                                            value: &self.source[start..self.index],
                                            is_int: false,
                                        }
                                    });
                                }
                            },
                            _ => {
                                return Some(Token {
                                        token_type: TokenType::Literal{
                                            value: &self.source[start..self.index],
                                            is_int: true,
                                        }
                                });
                            }
                        }
                    }
                }
            }
            Some(c @ 'A'..='Z') | Some(c @ '_'..='z') => {
                let start = self.index - c.len_utf8();
                loop {
                    match self.peek() {
                        Some('0'..='9') => {
                            self.next_char();
                        }
                        Some('A'..='Z') => {
                            self.next_char();
                        }
                        Some('_'..='z') => {
                            self.next_char();
                        }
                        _ =>{
                            let text = &self.source[start..self.index];
                            match text {
                                "for" => {
                                    return Some(Token {
                                        token_type: TokenType::KeyWord(KeyWord::For),
                                    });
                                }
                                "if" => {
                                    return Some(Token {
                                        token_type: TokenType::KeyWord(KeyWord::If),
                                    });
                                }
                                "else" => {
                                    return Some(Token {
                                        token_type: TokenType::KeyWord(KeyWord::Else),
                                    });
                                }
                                "fn" => {
                                    return Some(Token {
                                        token_type: TokenType::KeyWord(KeyWord::Fn),
                                    });
                                }
                                "print" => {
                                    return Some(Token {
                                        token_type: TokenType::KeyWord(KeyWord::Print),
                                    });
                                }
                                "input" => {
                                    return Some(Token {
                                        token_type: TokenType::KeyWord(KeyWord::Input),
                                    });
                                }
                                "return" => {
                                    return Some(Token {
                                        token_type: TokenType::KeyWord(KeyWord::Return),
                                    });
                                }
                                _ => {
                                    return Some(Token {
                                        token_type: TokenType::Identifier{
                                            name: text,
                                        },
                                    });
                                }
                                
                            }
                        },
                    }
                }
            }

            _ => (),
        }
        return None;
    }
}
