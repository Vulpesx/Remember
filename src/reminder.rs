use std::str::FromStr;

use anyhow::{bail, Context};
use libnotify::Notification;
use chrono::{DateTime, Local, Datelike, Timelike};


#[derive(Debug)]
pub struct Reminder<'a> {
    summary: &'a str,
    body: Option<&'a str>,
    when: When,
    done: bool,
    notif: Notification,
}

#[derive(Debug, Clone)]
pub enum When {
    Duration(u32),
    Day(String, u32, u32),
    Date(u32, u32, i32, u32, u32),
    Time(u32, u32),
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
                    self.when = When::Duration(s - 1);
                    false
                } else {
                    true
                }
            },
            Day(d, h, m) => {
                if d.to_lowercase() != now.weekday().to_string().to_lowercase() {
                    return false;
                }
                if now.hour() < h {
                    return false;
                }
                if now.minute() < m{
                    return false;
                }
                true
            }
            Date(d, m, y, h, min) => {
                if now.year() < y && y != -1 {
                    return false;
                }
                if now.month() < m && m != 0{
                    return false;
                }
                if now.day() < d {
                    return false;
                }
                if now.hour() < h{
                    return false;
                }
                if now.minute() < min{
                    return false;
                }
                true
            },
            Time(h, m) => {
                if now.hour() < h{
                    return false;
                }
                if now.minute() < m{
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

    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[derive(Debug)]
pub enum ReminderError<'a> {
    FromStr(&'a str),
}

//impl<'a> FromStr for Reminder<'a> {
    //type Err = anyhow::Error;
    //fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let v: Vec<&str> = s.split(";").collect();
        //if "montuewedthufrisatsun".contains(v[0]) {
            //if v.len() < 3 { bail!("expected atleast 3 arguments, but instead got: {}", v.len()) }
            //let day = v[0];
            //let t: Vec<&str> = v[1].split(":").collect();
            //if t.len() != 2 { bail!("invalid time! should be [hour]:[minute]") }
            //let h: u32 = t[0].parse().context("failed to parse hour")?;
            //let m: u32 = t[1].parse().context("failed to parse minute")?;
            //let sum = v[2];
            //let text = match v.len() {
                //x if x > 3 => { Some(v[3])},
                //_ => { None }
            //};
            //let day = When::Day(day.to_string(), Time::new(h, m));
        //} else if v[0].contains("/") {
            //todo!("parse dates")
        //} else if v[0].contains(":") {
            //todo!("parse times")
        //} else {
            //todo!("parse durations")
        //}

        //bail!("todo")
    //}
//}

#[macro_export]
macro_rules! remind {
    ($hour:literal:$minute:literal $sum:literal) => {
        Reminder::new(When::Time($hour, $minute), $sum, None)
    };
    ($hour:literal:$minute:literal $sum:literal $body:literal) => {
        Reminder::new($crate::When::Time($hour, $minute), $sum, Some($body))
    };
    ($day:ident $hour:literal:$minute:literal $sum:literal) => {
        {
            let day = stringify!($day).to_lowercase();
            if !"montuewedthufrisatsun".contains(&day) {
                panic!("invalid day");
            }
            Reminder::new($crate::When::Day(day, $hour, $minute), $sum, None)
        }
    };
    ($day:ident $hour:literal:$minute:literal $sum:literal $body:literal) => {
        {
            let day = stringify!($day).to_lowercase();
            if !"montuewedthufrisatsun".contains(&day) {
                panic!("invalid day");
            }
            Reminder::new($crate::When::Day(day, $hour, $minute), $sum, Some($body))
        }
    };
    ($sec:literal $sum:literal) => {
        Reminder::new($crate::When::Duration($sec), $sum, None)
    };
    ($sec:literal $sum:literal $body:literal) => {
        Reminder::new($crate::When::Duration($sec), $sum, Some($body))
    };
    ($day:literal/$month:literal/$year:literal $hour:literal:$minute:literal $sum:literal) => {
        Reminder::new($crate::When::Date($day, $month, $year, $hour, $minute), $sum, None)
    };
    ($day:literal/$month:literal/$year:literal $hour:literal:$minute:literal $sum:literal $body:literal) => {
        Reminder::new($crate::When::Date($day, $month, $year, $hour, $minute), $sum, Some($body))
    };
}

