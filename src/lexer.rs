#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
}
#[derive(Debug, PartialEq)]
enum TokenType {
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
    Literal,

    /* foo */
    Identifier,
}
#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType,
    token_str: &'a str,
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
                    "Token type: {:?}, Token str: {}",
                    token.token_type, token.token_str
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
                        token_str: "==",
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Assign,
                        token_str: "=",
                    });
                }
            },
            Some('+') => match self.peek() {
                Some('+') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Increment,
                        token_str: "++",
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Plus,
                        token_str: "+",
                    });
                }
            },
            Some('-') => match self.peek() {
                Some('-') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Decrement,
                        token_str: "--",
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Minus,
                        token_str: "-",
                    });
                }
            },
            Some('*') => {
                return Some(Token {
                    token_type: TokenType::Star,
                    token_str: "*",
                });
            }
            Some('/') => {
                return Some(Token {
                    token_type: TokenType::Slash,
                    token_str: "/",
                });
            }
            Some('(') => {
                return Some(Token {
                    token_type: TokenType::LeftParenthesis,
                    token_str: "(",
                });
            }
            Some(')') => {
                return Some(Token {
                    token_type: TokenType::RightParenthesis,
                    token_str: ")",
                });
            }
            Some('{') => {
                return Some(Token {
                    token_type: TokenType::LeftBraces,
                    token_str: "{",
                });
            }
            Some('}') => {
                return Some(Token {
                    token_type: TokenType::RightBraces,
                    token_str: "}",
                });
            }
            Some('[') => {
                return Some(Token {
                    token_type: TokenType::LeftBrackets,
                    token_str: "[",
                });
            }
            Some(']') => {
                return Some(Token {
                    token_type: TokenType::RightBrackets,
                    token_str: "]",
                });
            }
            Some('.') => {
                return Some(Token {
                    token_type: TokenType::Point,
                    token_str: ".",
                });
            }
            Some(':') => {
                return Some(Token {
                    token_type: TokenType::Colon,
                    token_str: ":",
                });
            }
            Some(',') => {
                return Some(Token {
                    token_type: TokenType::Comma,
                    token_str: ",",
                });
            }
            Some('|') => {
                return Some(Token {
                    token_type: TokenType::Pipe,
                    token_str: "|",
                });
            }
            Some('!') => match self.peek() {
                Some('=') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::NotEqual,
                        token_str: "!=",
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Exclamation,
                        token_str: "!",
                    });
                }
            },
            Some('<') => match self.peek() {
                Some('=') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::LessEq,
                        token_str: "<=",
                    });
                }
                Some('<') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::LeftShift,
                        token_str: "<<",
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Less,
                        token_str: "<",
                    });
                }
            },
            Some('>') => match self.peek() {
                Some('=') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::GreaterEq,
                        token_str: ">=",
                    });
                }
                Some('>') => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::RightShift,
                        token_str: ">>",
                    });
                }
                _ => {
                    return Some(Token {
                        token_type: TokenType::Greater,
                        token_str: ">",
                    });
                }
            },
            Some('%') => {
                return Some(Token {
                    token_type: TokenType::Percent,
                    token_str: "%",
                });
            }
            Some('&') => {
                return Some(Token {
                    token_type: TokenType::Ampersand,
                    token_str: "&",
                });
            }
            Some('^') => {
                return Some(Token {
                    token_type: TokenType::Caret,
                    token_str: "^",
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
                    token_str: "\n",
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
                                            token_type: TokenType::Literal,
                                            token_str: &self.source[start..self.index],
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
                                                    token_type: TokenType::Literal,
                                                    token_str: &self.source[start..self.index],
                                                });
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    return Some(Token {
                                        token_type: TokenType::Literal,
                                        token_str: &self.source[start..self.index],
                                    });
                                }
                            },
                            _ => {
                                return Some(Token {
                                    token_type: TokenType::Literal,
                                    token_str: &self.source[start..self.index],
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
                        _ => break,
                    }
                }
                return Some(Token {
                    token_type: TokenType::Identifier,
                    token_str: &self.source[start..self.index],
                });
            }

            _ => (),
        }
        return None;
    }
}
