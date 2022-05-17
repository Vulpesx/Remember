use std::{thread, time, vec};

use chrono::{DateTime, Local, Datelike, Timelike};
use libnotify::Notification;

#[derive(Debug)]
struct Reminder<'a> {
    summary: &'a str,
    body: Option<&'a str>,
    when: When,
    done: bool,
    notif: Notification,
}

#[derive(Debug, Clone)]
struct Time {
    hour: u32,
    minutes: u32,
}

impl Time {
    fn new(hour: u32, minutes: u32) -> Time {
        Time { hour, minutes }
    }
}

#[derive(Debug, Clone)]
struct Date {
    day: u32,
    month: u32,
    year: i32,
}

impl Date {
    fn new(day: u32, month: u32, year: i32) -> Date {
        Date { day, month, year }
    }
}

#[derive(Debug, Clone)]
enum When {
    Duration(u32),
    Day(String, Time),
    Date(Date, Time),
}

impl<'a> Reminder<'a> {
    pub fn new<'s>(when: When, summary: &'s str, body: Option<&'s str>) -> Reminder<'s> {
        Reminder {
            when,
            summary,
            body,
            notif: Notification::new(summary, body, None),
            done: false,
        }
    }

    pub fn check(&mut self, now: DateTime<Local>) -> bool {
        use When::*;
        match self.when.clone() {
            Duration(s) => {
                if s > 0 {
                    println!("{}", s);
                    self.when = When::Duration(s - 1);
                    false
                } else {
                    true
                }
            },
            Day(d, t) => {
                if d.to_lowercase() != now.weekday().to_string().to_lowercase() {
                    return false;
                }
                if now.hour() < t.hour {
                    return false;
                }
                if now.minute() < t.minutes {
                    return false;
                }
                true
            }
            Date(d, t) => {
                if now.year() < d.year && d.year != 0 {
                    return false;
                }
                if now.month() < d.month {
                    return false;
                }
                if now.day() < d.day {
                    return false;
                }
                if now.hour() < t.hour {
                    return false;
                }
                if now.minute() < t.minutes {
                    return false;
                }
                true
            }
        }
    }

    pub fn show(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", self);
        self.done = true;
        Ok(self.notif.show()?)
    }
}

fn main() {
    libnotify::init("Remember");

    let mut day  = Reminder::new(When::Day("Tue".to_string(), Time::new(11, 25)), "this is a day test", None);
    let mut date  = Reminder::new(When::Date(Date::new(16, 5, 2022), Time::new(11, 25)), "this is a date test", None);
    let mut duration  = Reminder::new(When::Duration(3), "this is a duration test", None);
    let mut url = Reminder::new(When::Duration(2), "url test", Some("<https://google.com>"));

    let mut reminders = vec![day, date, duration, url];
    let mut len = reminders.len();
    let mut quit = false;

    while reminders.len() > 0 && !quit{
        thread::sleep(time::Duration::from_secs(1));
        let now = Local::now();
        for i in 0..len {
            let mut r = &mut reminders[i];
            if !r.done && r.check(now) {
                r.show().unwrap();
            }
        }
    }

    libnotify::uninit();
}
