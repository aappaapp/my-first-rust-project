use std::{thread, time::Duration};

pub fn sleep(time: u64) {
    thread::sleep(Duration::from_millis(time.into()));
}
