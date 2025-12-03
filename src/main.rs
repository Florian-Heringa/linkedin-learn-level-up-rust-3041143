use chrono::{Duration, TimeZone};
use chrono::{DateTime, Local, NaiveDateTime, NaiveDate, NaiveTime};

struct ImportantEvent {
    what: String,
    when: DateTime<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::now()
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Local.from_local_datetime(&NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2020, 12, 25).unwrap(), 
            NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        )).single().unwrap(),
    };
    
    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::now() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::now() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}

