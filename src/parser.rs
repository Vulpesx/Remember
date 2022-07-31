use crate::{
    lexer::{Lexer, Loc, Token, TokenKind},
    Reminder,
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
        Err(token) => Err(ParserError::UnexpectedToken(
            token.loc,
            token.kind,
            token.text,
            TokenKind::Num,
        )),
    }
}

pub fn parse_time<C: Iterator<Item = char>>(lexer: &mut Lexer<C>) -> Result<Reminder, ParserError> {
    match lexer.expect_token(TokenKind::Num) {
        Ok(token) => {
            let hour = token.text.parse::<u32>().unwrap();
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
        Err(token) => Err(ParserError::UnexpectedToken(
            token.loc,
            token.kind,
            token.text,
            TokenKind::Num,
        )),
    }
}

pub enum ParserError {
    NoToken,
    UnexpectedToken(Loc, TokenKind, String, TokenKind), // first is the unexpected token and its text then the expected token
    UnclosedStr(Loc, String),
}

pub enum Command {
    Quit,
    Remind,
    List,
    Edit,
    Help,
    Invalid(Option<Token>),
}
