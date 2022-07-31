use std::{
    env,
    io::{self, Write},
    thread, time, vec,
};

use chrono::Local;

use remember::{
    parser::{Command, ParserError},
    *,
};

use lexer::Lexer;

enum Mode {
    Normal,
    Deamon,
    DebugNew,
    DebugParser,
    DebugLexer,
}

struct Config {
    file_path: Option<String>,
    mode: Mode,
}

impl Config {
    fn from_iter(args: &mut impl Iterator<Item = String>) -> Config {
        args.next().expect("Program name should allways be present");
        let mut config = Self {
            file_path: None,
            mode: Mode::Normal,
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--debug" => {
                    if let Some(mode) = args.next() {
                        match mode.as_str() {
                            "parser" => config.mode = Mode::DebugParser,
                            "lexer" => config.mode = Mode::DebugLexer,
                            "new" => config.mode = Mode::DebugNew,
                            _ => {
                                eprintln!("ERROR :: unknown value for debug mode");
                                std::process::exit(1);
                            }
                        }
                    } else {
                        eprintln!("ERROR :: no value given for debug mode");
                        std::process::exit(1)
                    }
                }

                x if x == "--deamon" || x == "-d" => config.mode = Mode::Deamon,

                other => {
                    if config.file_path.is_none() {
                        config.file_path = Some(other.to_string());
                    } else {
                        eprintln!("ERROR :: file path already provided");
                        std::process::exit(1);
                    }
                }
            }
        }

        config
    }
}

fn print_help() {
    println!("Remember");
    println!("Desc - reminds you of things");
    println!("usage - remember [options] [config]");
    println!("options:");
    println!("  -d --deamon     start in deamon mode");
    println!("  --debug <mode>  start in specifide debug mode");
}

fn normal(config: Config) {
    println!("starting in normal mode");
}

fn deamon(config: Config) {
    println!("starting in deamon mode");
}

fn debug_new(config: Config) {
    println!("starting in debug mode");
}

fn debug_parser(config: Config) {
    println!("starting in debug parser mode");

    let mut buf = String::new();
    let input = io::stdin();

    let mut reminders: Vec<Reminder> = vec![];

    loop {
        print!(">");
        io::stdout().flush();
        input.read_line(&mut buf).unwrap();

        let mut lexer = Lexer::new(buf.chars(), None);

        let command = parser::get_command(&mut lexer);

        match command {
            Some(c) => match c {
                Command::Quit => {
                    println!("quiting");
                    std::process::exit(0);
                }
                Command::List => {
                    if reminders.len() > 0 {
                        println!("id || Reminder");
                        for (i, r) in reminders.iter().enumerate() {
                            println!("{}    {:?}", i, r);
                        }
                    } else {
                        println!("no reminders set");
                    }
                }
                Command::Remind => match parser::parse_time(&mut lexer) {
                    Ok(r) => {
                        reminders.push(r);
                    }
                    Err(e) => match e {
                        ParserError::NoToken => eprintln!("no input"),
                        ParserError::UnclosedStr(loc, text) => {
                            print!(" {}", " ".repeat(loc.col));
                            println!("{}", "^".repeat(text.len() - 1));
                            println!("ERROR :: UnclosedStr");
                        }
                        ParserError::UnexpectedToken(loc, got, text, expected) => {
                            print!("{}", " ".repeat(loc.col));
                            println!("^^^");
                            println!("ERROR :: UnexpectedToken");
                            println!("GOT :: {:?} : {}", got, text);
                            println!("EXPECTED:: {:?}", expected);
                        }
                    },
                },
                Command::Edit => {
                    println!("TODO!!");
                }
                Command::Help => {
                    print_help();
                }
                Command::Invalid(o) => match o {
                    Some(t) => {
                        println!("{}{}", " ".repeat(t.loc.col), "^".repeat(t.text.len()));
                        println!("ERROR :: invalid command '{}'", t.text);
                    }
                    None => {
                        let t = lexer.peek_token();
                        println!("{}{}", " ".repeat(t.loc.col), "^".repeat(t.text.len()));
                        println!("ERROR :: invalid command '{}'", t.text);
                    }
                },
            },
            None => {}
        }

        buf.clear();
    }
}

fn debug_lexer(config: Config) {
    println!("starting in debug lexer mode");

    let mut buf = String::new();
    let input = io::stdin();

    loop {
        print!(">");
        io::stdout().flush();
        input.read_line(&mut buf).unwrap();

        let mut lexer = Lexer::new(buf.chars(), None);
        for token in lexer {
            println!("{:?}", token);
        }

        buf.clear();
    }
}

fn main() {
    let config = Config::from_iter(&mut env::args());

    match config.mode {
        Mode::Normal => normal(config),
        Mode::Deamon => deamon(config),
        Mode::DebugNew => debug_new(config),
        Mode::DebugParser => debug_parser(config),
        Mode::DebugLexer => debug_lexer(config),
    }
}

fn main2() {
    libnotify::init("Remember");

    let time = remind!(10:30 "this is a time test" "ljlj"); // Reminder::new(When::Time(10, 30), "this is a time test", Some("ljlj"))
    let day = remind!(wed 11:25 "this is a day test"); // Reminder::new(When::Day("Tue".to_string(), 11, 25), "this is a day test", None);
    let date = remind!(16/5/2022 11:25 "this is a date test"); // Reminder::new(When::Date(16, 5, 2022, 11, 25), "this is a date test", None);
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
