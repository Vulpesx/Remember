use std::{thread, time, ops::Rem, vec};

use chrono::{Utc, DateTime, Local, Datelike, Timelike};
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
                if now.minute() < t.minutes && now.hour() < t.hour {
                    print!("klashfkj");
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
                if now.minute() < t.minutes && now.hour() < t.hour{
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

    let mut r = Reminder::new(When::Day("Mon".to_string(), Time::new(16, 58)), "this is a test", Some("this is a test of the amazinf Remember program"));
    let mut reminders = vec![r];

    while reminders.len() > 0 {
        thread::sleep(time::Duration::from_secs(1));
        let now = Local::now();
        for i in 0..reminders.len() {
            let mut r = &mut reminders[i];
            if r.check(now) {
                r.show().unwrap();
            }
            if r.done {
                reminders.remove(i);
            }
        }
    }

    libnotify::uninit();
}
