use std::{thread, time, vec};

use chrono::Local;
use remember::{Reminder, remind};

fn main() {
    libnotify::init("Remember");

    let time = remind!(10:30 "this is a time test" "ljlj");
    let day = remind!(wed 11:25 "this is a day test");          // Reminder::new(When::Day("Tue".to_string(), Time::new(11, 25)), "this is a day test", None);
    let date = remind!(16/5/2022 11:25 "this is a date test");  // Reminder::new(When::Date(Date::new(16, 5, 2022), Time::new(11, 25)), "this is a date test", None);
    let duration = remind!(3 "this is a duration test");        // Reminder::new(When::Duration(3), "this is a duration test", None);
    let url = remind!(3 "<https://google.com>");                // Reminder::new(When::Duration(2), "url test", Some("<https://google.com>"));

    let mut reminders = vec![time, day, date, duration, url];
    let mut quit = false;

    while reminders.len() > 0 && !quit{
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
