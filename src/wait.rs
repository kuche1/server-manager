use chrono::NaiveTime; // cargo add chrono
use chrono::offset::Local;
use std::thread;
use std::time::Duration;

pub fn main(restart_at: u8, check_time_sleep_sec: u64) {
    let target = NaiveTime::from_hms_opt(restart_at.into(), 0, 0).unwrap();

    loop {
        let now = Local::now().time();

        println!("{target} >? {now}");

        if now > target {
            println!(
                "too late for a restart; sleeping {} sec",
                check_time_sleep_sec
            );
            thread::sleep(Duration::from_secs(check_time_sleep_sec));
        } else {
            break;
        }
    }

    loop {
        let now = Local::now().time();

        println!("{target} <? {now}");

        if now < target {
            println!(
                "too early for a restart; sleeping {} sec",
                check_time_sleep_sec
            );
            thread::sleep(Duration::from_secs(check_time_sleep_sec));
        } else {
            break;
        }
    }
}
