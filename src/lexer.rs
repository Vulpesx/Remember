use std::iter::Peekable;

pub struct Lexer<Chars: Iterator<Item = char>> {
    chars: Peekable<Chars>,
    exhausted: bool,
    peeked: Option<Token>
}

impl<Chars: Iterator<Item = char>> Lexer<Chars> {
    pub fn new(chars: Chars) -> Self {
        Self {
            chars: chars.peekable(),
            exhausted: false,
            peeked: None,
        }
    }

    pub fn expect_token(&mut self, kind: TokenKind) -> Result<Token, Token> {
        let token = self.next_token();
        if kind == token.kind {
            Ok(token)
        } else {
            Err(token)
        }
    }

    pub fn peek_token(&mut self) -> &Token {
        let token = self.next_token();
        self.peeked.insert(token)
    }

    pub fn next_token(&mut self) -> Token {
        self.peeked.take().unwrap_or(self.chop_tokens_from_chars())
    }

    fn trim_whitespace(&mut self) {
        while self.chars.next_if(|x| x.is_whitespace() && *x != '\n').is_some() {}
    }

    fn drop_line(&mut self) {
        while self.chars.next_if(|x| *x != '\n').is_some() {}
        if self.chars.next_if(|x| *x == '\n').is_some() {}
    }

    fn chop_tokens_from_chars(&mut self) -> Token {
        assert!(!self.exhausted, "Completely exhausted Lexer. Caller shouldnt trie to pull from Lexer after exhaustion!");

        self.trim_whitespace();

        while let Some(x) = self.chars.peek() {
            if *x != '\n' && *x != '#' {
                break
            }

            self.drop_line();
            self.trim_whitespace();
        }

        match self.chars.next() {
            Some(x) => {
                let mut text = x.to_string();
                match x {
                    ':' =>  {Token{kind: TokenKind::Colon,     text}},
                    ';' =>  {Token{kind: TokenKind::Semicolon, text}},
                    '"' =>  {
                        text.clear();
                        while let Some(x) = self.chars.next_if(|x| *x != '"') {
                            text.push(x);
                        }
                        Token{
                            kind: if self.chars.next_if(|x| *x == '"').is_some() {
                                TokenKind::Str
                            } else {
                                TokenKind::UnclosedStr
                            },
                            text
                        }
                    }
                    '\'' => {
                        text.clear();
                        while let Some(x) = self.chars.next_if(|x| *x != '\'') {
                            text.push(x);
                        }
                        Token{
                            kind: if self.chars.next_if(|x| *x == '\'').is_some() {
                                TokenKind::Str
                            } else {
                                TokenKind::UnclosedStr
                            },
                            text
                        }
                    }
                    x if x.is_alphabetic() =>  {
                        if !valid_char(&x) {
                            self.exhausted = true;
                            Token{kind: TokenKind::Invalid, text}
                        } else {
                            while let Some(x) = self.chars.next_if(valid_char) {
                                text.push(x);
                            }

                            Token{kind: TokenKind::Str, text}
                        } 
                    },
                    x if x.is_numeric() => {
                        if !valid_char(&x) {
                            self.exhausted = true;
                            Token{kind: TokenKind::Invalid, text}
                        } else {
                            while let Some(x) = self.chars.next_if(|x| x.is_numeric() || x.is_whitespace()) {
                                if !x.is_whitespace() {
                                    text.push(x);
                                }
                            }

                            Token{kind: TokenKind::Num, text}
                        }
                    }
                    _ => {
                        self.exhausted = true;
                        Token{kind: TokenKind::Invalid, text}
                    }
                }
            },
            None => {
                self.exhausted = true;
                Token{kind: TokenKind::End, text: "".to_string()}
            },
        }
    }
}

fn valid_char(x: &char) -> bool {
        x.is_alphanumeric()
}

impl<Chars: Iterator<Item = char>> Iterator for Lexer<Chars> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            None
        } else {
            Some(self.next_token())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    //values
    Str,
    Num,

    //sybols
    Colon,
    Semicolon,

    //Terminators
    UnclosedStr,
    Invalid,
    End,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}
