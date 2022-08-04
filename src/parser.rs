use crate::{
    lexer::{Lexer, Loc, Token, TokenKind},
    loc_here, Reminder,
};

pub fn get_command<C: Iterator<Item = char>>(lexer: &mut Lexer<C>) -> Option<Command> {
    let token = lexer.next();
    match token {
        Some(token) => match token.kind {
            TokenKind::Quit => Some(Command::Quit),
            TokenKind::Remind => Some(Command::Remind),
            TokenKind::List => Some(Command::List),
            TokenKind::Edit => Some(Command::Edit),
            TokenKind::Help => Some(Command::Help),
            _ => Some(Command::Invalid(lexer.next())),
        },
        None => None,
    }
}

pub fn parse_duration<C: Iterator<Item = char>>(
    lexer: &mut Lexer<C>,
) -> Result<Reminder, ParserError> {
    match lexer.expect_token(TokenKind::Num) {
        Ok(token) => {
            let duration = token.text.parse::<u32>().unwrap();
            let summary = match lexer.expect_token(TokenKind::Str) {
                Ok(t) => t.text,
                Err(t) => match t {
                    Token {
                        kind: TokenKind::End,
                        ..
                    } => String::from("no summary provided"),
                    Token {
                        kind: TokenKind::UnclosedStr,
                        text,
                        loc,
                    } => return Err(ParserError::UnclosedStr(loc, text)),
                    Token { kind, text, loc } => {
                        return Err(ParserError::UnexpectedToken(
                            loc,
                            kind,
                            text,
                            TokenKind::Str,
                        ));
                    }
                },
            };
            let description = match lexer.next() {
                Some(t) => match t {
                    Token {
                        kind: TokenKind::Str,
                        text,
                        ..
                    } => Some(text),
                    Token {
                        kind: TokenKind::UnclosedStr,
                        text,
                        loc,
                    } => {
                        return Err(ParserError::UnclosedStr(loc, text));
                    }
                    _ => None,
                },
                None => None,
            };
            Ok(Reminder::new(
                crate::When::Duration(duration),
                summary,
                description,
            ))
        }
        Err(token) => match token {
            Token {
                kind: TokenKind::UnclosedStr,
                text,
                loc,
            } => Err(ParserError::UnclosedStr(loc, text)),
            Token {
                kind: TokenKind::End,
                loc,
                ..
            } => Err(ParserError::NoToken(loc)),
            Token { kind, text, loc } => Err(ParserError::UnexpectedToken(
                loc,
                kind,
                text,
                TokenKind::Str,
            )),
        },
    }
}

pub fn parse_time<C: Iterator<Item = char>>(lexer: &mut Lexer<C>) -> Result<Reminder, ParserError> {
    match lexer.expect_token(TokenKind::Num) {
        Ok(token) => {
            let hour = token.text.parse::<u32>().unwrap();
            if hour > 12 {
                return Err(ParserError::InvalidNum(token.loc, hour as i32, 0, 12));
            }
            let minute = match lexer.expect_token(TokenKind::Num) {
                Ok(token) => token.text.parse::<u32>().unwrap(),
                Err(token) => {
                    return Err(ParserError::UnexpectedToken(
                        token.loc,
                        token.kind,
                        token.text,
                        TokenKind::Num,
                    ))
                }
            };
            if minute > 59 {
                return Err(ParserError::InvalidNum(token.loc, minute as i32, 0, 59));
            }
            let summary = match lexer.expect_token(TokenKind::Str) {
                Ok(token) => token.text,
                Err(token) => match token {
                    Token {
                        kind: TokenKind::End,
                        ..
                    } => String::from("no summary provided"),
                    Token {
                        kind: TokenKind::UnclosedStr,
                        text,
                        loc,
                    } => return Err(ParserError::UnclosedStr(loc, text)),
                    Token { kind, text, loc } => {
                        return Err(ParserError::UnexpectedToken(
                            loc,
                            kind,
                            text,
                            TokenKind::Str,
                        ))
                    }
                },
            };
            let description = match lexer.next() {
                Some(token) => match token {
                    Token {
                        kind: TokenKind::Str,
                        text,
                        ..
                    } => Some(text),
                    Token {
                        kind: TokenKind::UnclosedStr,
                        text,
                        loc,
                    } => return Err(ParserError::UnclosedStr(loc, text)),
                    _ => None,
                },
                None => None,
            };
            Ok(Reminder::new(
                crate::When::Time(hour, minute),
                summary,
                description,
            ))
        }

        Err(token) => match token {
            Token {
                kind: TokenKind::UnclosedStr,
                text,
                loc,
            } => Err(ParserError::UnclosedStr(loc, text)),
            Token {
                kind: TokenKind::End,
                loc,
                ..
            } => Err(ParserError::NoToken(loc)),
            Token { kind, text, loc } => Err(ParserError::UnexpectedToken(
                loc,
                kind,
                text,
                TokenKind::Str,
            )),
        },
    }
}

pub fn parse_day<C: Iterator<Item = char>>(lexer: &mut Lexer<C>) -> Result<Reminder, ParserError> {
    match lexer.expect_token(TokenKind::Str) {
        Ok(token) => {
            let day = token.text;
            if !"montuewedthufrisatsun".contains(&day) {
                return Err(ParserError::InvalidDay(token.loc, day));
            }
            let hour = match lexer.expect_token(TokenKind::Num) {
                Ok(token) => token.text.parse::<u32>().unwrap(),
                Err(token) => {
                    return Err(ParserError::UnexpectedToken(
                        token.loc,
                        token.kind,
                        token.text,
                        TokenKind::Num,
                    ))
                }
            };
            if hour > 12 {
                return Err(ParserError::InvalidNum(lexer.loc(), hour as i32, 0, 12));
            }
            let minute = match lexer.expect_token(TokenKind::Num) {
                Ok(token) => token.text.parse::<u32>().unwrap(),
                Err(token) => {
                    return Err(ParserError::UnexpectedToken(
                        token.loc,
                        token.kind,
                        token.text,
                        TokenKind::Num,
                    ))
                }
            };
            if minute > 59 {
                return Err(ParserError::InvalidNum(lexer.loc(), minute as i32, 0, 59));
            }
            let summary = match lexer.expect_token(TokenKind::Str) {
                Ok(token) => token.text,
                Err(token) => match token {
                    Token {
                        kind: TokenKind::UnclosedStr,
                        text,
                        loc,
                    } => return Err(ParserError::UnclosedStr(loc, text)),
                    Token {
                        kind: TokenKind::End,
                        ..
                    } => String::from("No summary provided"),
                    Token { kind, text, loc } => {
                        return Err(ParserError::UnexpectedToken(
                            loc,
                            kind,
                            text,
                            TokenKind::Str,
                        ))
                    }
                },
            };
            let description = match lexer.next() {
                Some(token) => match token {
                    Token {
                        kind: TokenKind::Str,
                        text,
                        ..
                    } => Some(text),
                    Token {
                        kind: TokenKind::UnclosedStr,
                        text,
                        loc,
                    } => return Err(ParserError::UnclosedStr(loc, text)),
                    _ => None,
                },
                None => None,
            };
            Ok(Reminder::new(
                crate::When::Day(day, hour, minute),
                summary,
                description,
            ))
        }
        Err(token) => match token {
            Token {
                kind: TokenKind::UnclosedStr,
                text,
                loc,
            } => Err(ParserError::UnclosedStr(loc, text)),
            Token {
                kind: TokenKind::End,
                loc,
                ..
            } => Err(ParserError::NoToken(loc)),
            Token { kind, text, loc } => Err(ParserError::UnexpectedToken(
                loc,
                kind,
                text,
                TokenKind::Str,
            )),
        },
    }
}

pub enum ParserError {
    NoToken(Loc),
    UnexpectedToken(Loc, TokenKind, String, TokenKind), //found, text of token, expected
    UnclosedStr(Loc, String),
    InvalidDay(Loc, String),
    InvalidNum(Loc, i32, i32, i32), //num found, min, max
}

pub enum Command {
    Quit,
    Remind,
    List,
    Edit,
    Help,
    Invalid(Option<Token>),
}
