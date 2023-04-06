use std::io::{stdin, stdout, Read, Write};

use crate::utils::sleep;

pub fn print_dialog(string: &str) {
    print_dialog_with_custom_time(string, 15);
}

pub fn print_dialog_new_line(string: &str) {
    print_dialog_new_line_with_custom_time(string, 15);
}

pub fn print_dialog_with_custom_time(string: &str, time: u64) {
    for letter in string.split("") {
        print!("{}", letter);
        stdout().flush().unwrap();
        sleep(time);
    }
}

pub fn print_dialog_new_line_with_custom_time(string: &str, time: u64) {
    for letter in string.split("") {
        print!("{}", letter);
        stdout().flush().unwrap();
        sleep(time);
    }
    print!("\n");
}

pub fn pause() {
    stdin().read(&mut [0u8]).unwrap();
}
