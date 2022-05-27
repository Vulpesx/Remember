use std::{iter::Peekable, thread, time, vec};

use chrono::Local;
use remember::{remind, Reminder};

struct Lexer<Chars: Iterator<Item = char>> {
    chars: Peekable<Chars>,
    exhausted: bool,
    peeked: Option<Token>
}

impl<Chars: Iterator<Item = char>> Lexer<Chars> {
    fn new(chars: Chars) -> Self {
        Self {
            chars: chars.peekable(),
            exhausted: false,
            peeked: None,
        }
    }

    fn next_token(&mut self) -> Token {
        self.peeked.take().unwrap_or(self.chop_tokens_from_chars())
    }

    

    fn chop_tokens_from_chars(&mut self) -> Token {
        assert!(!self.exhausted, "Completely exhausted Lexer. Caller shouldnt trie to pull from Lexer after exhaustion!");
        let mut cnum = 0;

        while self.chars.next_if(|x| x.is_whitespace() && *x != '\n').is_some() {}

        match self.chars.next() {
            Some(x) => {
                let mut text = x.to_string();
                match x {
                    ':' =>  {Token{kind: TokenKind::Colon,     text: text}},
                    ';' =>  {Token{kind: TokenKind::Semicolon, text: text}},

                    x if x.is_alphabetic() =>  {
                        if !valid_char(&x) {
                            self.exhausted = true;
                            Token{kind: TokenKind::Invalid, text: text}
                        } else {
                            while let Some(x) = self.chars.next_if(valid_char) {
                                text.push(x);
                            }

                            Token{kind: TokenKind::Str, text: text}
                        } 
                    },
                    x if x.is_numeric() => {
                        if !valid_char(&x) {
                            self.exhausted = true;
                            Token{kind: TokenKind::Invalid, text: text}
                        } else {
                            while let Some(x) = self.chars.next_if(valid_char) {
                                text.push(x);
                            }

                            Token{kind: TokenKind::Num, text: text}
                        }
                    }
                    _ => {
                        self.exhausted = true;
                        Token{kind: TokenKind::Invalid, text: text}
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

#[derive(Debug)]
enum TokenKind {
    //values
    Str,
    Num,

    //sybols
    Colon,
    Semicolon,

    //Terminators
    Invalid,
    End,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    text: String,
}

fn main() {
    for token in Lexer::new("sun; 12:30; lkasdjflkj; lkjlkjlkjklj".chars()) {
        println!("{:?}", token);
    }
}

fn main2() {
    libnotify::init("Remember");

    let time = remind!(10:30 "this is a time test" "ljlj");
    let day = remind!(wed 11:25 "this is a day test"); // Reminder::new(When::Day("Tue".to_string(), Time::new(11, 25)), "this is a day test", None);
    let date = remind!(16/5/2022 11:25 "this is a date test"); // Reminder::new(When::Date(Date::new(16, 5, 2022), Time::new(11, 25)), "this is a date test", None);
    let duration = remind!(3 "this is a duration test"); // Reminder::new(When::Duration(3), "this is a duration test", None);
    let url = remind!(3 "<https://google.com>"); // Reminder::new(When::Duration(2), "url test", Some("<https://google.com>"));

    let mut reminders = vec![time, day, date, duration, url];
    let mut quit = false;

    while reminders.len() > 0 && !quit {
        thread::sleep(time::Duration::from_secs(1));
        let now = Local::now();
        for i in 0..reminders.len() {
            let mut r = &mut reminders[i];
            if !r.is_done() && r.check(now) {
                r.show().unwrap();
            }
        }
    }

    libnotify::uninit();
}
