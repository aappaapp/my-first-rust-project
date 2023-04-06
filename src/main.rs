mod io;
mod utils;

use io::*;

fn main() {
    intro()
}

fn intro() {
    print_dialog("Hello!");
    pause();
    print_dialog("\nWAIT...");
    print_dialog("WHO ");
    print_dialog_with_custom_time("ARE ", 50);
    print_dialog_with_custom_time("YOU?", 100);
}
