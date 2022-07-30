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
    debug: bool,
) -> Result<Reminder, ParserError> {
    match lexer.expect_token(TokenKind::Num) {
        Ok(token) => {
            let dur = token.text.parse::<u32>().unwrap();
            let sum = match lexer.expect_token(TokenKind::Str) {
                Ok(t) => t.text,
                Err(t) => match t {
                    Token {
                        kind: TokenKind::End,
                        loc,
                        ..
                    } => String::from("place holder summary"),
                    Token {
                        kind: TokenKind::UnclosedStr,
                        text,
                        loc,
                    } => return Err(ParserError::UnclosedStr(loc, text)),
                    Token { kind, text, loc } => {
                        return Err(ParserError::UnexpectedToken(loc, kind, TokenKind::Str));
                    }
                },
            };
            let desc = match lexer.next() {
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
            Ok(Reminder::new(crate::When::Duration(dur), sum, desc))
        }
        Err(token) => Err(ParserError::UnexpectedToken(
            token.loc,
            token.kind,
            TokenKind::Num,
        )),
    }
}

pub enum ParserError {
    NoToken,
    UnexpectedToken(Loc, TokenKind, TokenKind), // first is the unexpected token then the expected token
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
