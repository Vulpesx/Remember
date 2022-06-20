use crate::lexer::{Lexer, TokenKind, Token};

pub fn get_command<C: Iterator<Item = char>>(lexer: &mut Lexer<C>) -> Option<Command>{
    let token = lexer.peek_token();
    match token.kind {
        TokenKind::Quit => Some(Command::Quit),
        TokenKind::Remind => Some(Command::Remind),
        TokenKind::List => Some(Command::List),
        TokenKind::Edit => Some(Command::Edit),
        TokenKind::Help => Some(Command::Help),
        _ => Some(Command::Invalid(lexer.next())),
    }
}

pub enum Command {
    Quit,
    Remind,
    List,
    Edit,
    Help,
    Invalid(Option<Token>),
}

