use std::fmt::Display;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct Loc {
    pub file_path: Option<String>,
    pub row: usize,
    pub col: usize,
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file_path {
            Some(file_path) => write!(f, "{}:{}:{}", file_path, self.row, self.col),
            None => write!(f, "{}:{}", self.row, self.col),
        }
    }
}

#[macro_export]
macro_rules! loc_here {
    () => {
        Loc {
            file_path: Some(file!().to_string),
            row: line!() as usize,
            col: column!() as usize,
        }
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    //values
    Str,
    Num,

    //keywords
    Quit,
    Remind,
    List,
    Edit,
    Help,

    //sybols
    Colon,
    Semicolon,

    //Terminators
    UnclosedStr,
    Invalid,
    End,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub loc: Loc,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.text, self.loc)
    }
}

pub struct Lexer<Chars: Iterator<Item = char>> {
    chars: Peekable<Chars>,
    exhausted: bool,
    peeked: Option<Token>,
    file_path: Option<String>,
    lnum: usize,
    cnum: usize,
    bol: usize,
}

impl<Chars: Iterator<Item = char>> Lexer<Chars> {
    pub fn new(chars: Chars, file_path: Option<String>) -> Self {
        Self {
            chars: chars.peekable(),
            exhausted: false,
            peeked: None,
            file_path,
            lnum: 0,
            cnum: 0,
            bol: 0,
        }
    }

    pub fn loc(&self) -> Loc {
        Loc {
            file_path: self.file_path.clone(),
            row: self.lnum,
            col: self.cnum - self.bol + 1,
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
        self.peeked.get_or_insert(token)
    }

    fn next_token(&mut self) -> Token {
        self.peeked.take().unwrap_or_else(|| self.chop_tokens())
    }

    fn trim_whitespace(&mut self) {
        while self
            .chars
            .next_if(|x| x.is_whitespace() && *x != '\n')
            .is_some()
        {
            self.cnum += 1;
        }
    }

    fn drop_line(&mut self) {
        while self.chars.next_if(|x| *x != '\n').is_some() {
            self.cnum += 1;
        }
        if self.chars.next_if(|x| *x == '\n').is_some() {
            self.cnum += 1;
            self.lnum += 1;
            self.bol = self.cnum;
        }
    }

    fn chop_tokens(&mut self) -> Token {
        assert!(
            !self.exhausted,
            "Completely exhausted Lexer. Caller shouldnt try to pull from Lexer after exhaustion!"
        );

        self.trim_whitespace();

        while let Some(x) = self.chars.peek() {
            if *x != '\n' && *x != '#' {
                break;
            }

            self.drop_line();
            self.trim_whitespace();
        }

        let loc = self.loc();

        match self.chars.next() {
            Some(x) => {
                self.cnum += 1;
                let mut text = x.to_string();
                match x {
                    '"' => {
                        text.clear();
                        while let Some(x) = self.chars.next_if(|x| *x != '"') {
                            self.cnum += 1;
                            text.push(x);
                        }
                        Token {
                            kind: if self.chars.next_if(|x| *x == '"').is_some() {
                                TokenKind::Str
                            } else {
                                TokenKind::UnclosedStr
                            },
                            text,
                            loc,
                        }
                    }
                    '\'' => {
                        text.clear();
                        while let Some(x) = self.chars.next_if(|x| *x != '\'') {
                            self.cnum += 1;
                            text.push(x);
                        }
                        Token {
                            kind: if self.chars.next_if(|x| *x == '\'').is_some() {
                                TokenKind::Str
                            } else {
                                TokenKind::UnclosedStr
                            },
                            text,
                            loc,
                        }
                    }
                    x if x.is_alphabetic() => {
                        if !x.is_alphabetic() {
                            self.exhausted = true;
                            Token {
                                kind: TokenKind::Invalid,
                                text,
                                loc,
                            }
                        } else {
                            while let Some(x) = self.chars.next_if(|x| x.is_alphabetic()) {
                                self.cnum += 1;
                                text.push(x);
                            }

                            Token {
                                kind: match &*text.to_lowercase() {
                                    "quit" => TokenKind::Quit,
                                    "q" => TokenKind::Quit,
                                    "exit" => TokenKind::Quit,

                                    "edit" => TokenKind::Edit,
                                    "e" => TokenKind::Edit,

                                    "remind" => TokenKind::Remind,
                                    "r" => TokenKind::Remind,

                                    "list" => TokenKind::List,
                                    "ls" => TokenKind::List,

                                    "help" => TokenKind::Help,
                                    "h" => TokenKind::Help,

                                    _ => TokenKind::Str,
                                },
                                text,
                                loc,
                            }
                        }
                    }
                    x if x.is_numeric() => {
                        if !x.is_numeric() {
                            self.exhausted = true;
                            Token {
                                kind: TokenKind::Invalid,
                                text,
                                loc,
                            }
                        } else {
                            while let Some(x) = self.chars.next_if(|x| x.is_numeric()) {
                                self.cnum += 1;
                                text.push(x);
                            }

                            Token {
                                kind: TokenKind::Num,
                                text,
                                loc,
                            }
                        }
                    }
                    _ => {
                        self.exhausted = true;
                        Token {
                            kind: TokenKind::Invalid,
                            text,
                            loc,
                        }
                    }
                }
            }
            None => {
                self.exhausted = true;
                Token {
                    kind: TokenKind::End,
                    text: "".to_string(),
                    loc,
                }
            }
        }
    }
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
