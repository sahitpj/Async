use colors::ANSI_BOLD;
use colors::ANSI_COLOR_CYAN;
use colors::RESET;
use colors::GREEN;
use colors::RED;

use std::env;


pub fn  print_prompt(last_exit_status: bool) {
    let path = env::current_dir().unwrap();
    print!(
        "{}ASYNC {}{}{}  ",
        ANSI_BOLD,
        ANSI_COLOR_CYAN,
        path.display(),
        RESET
    );
    if last_exit_status {
        print!(
            "{}{}\u{2ba1}{}  ",
            ANSI_BOLD,
            GREEN,
            RESET
        );
    } else {
        print!(
            "{}{}\u{2ba1}{}  ",
            ANSI_BOLD,
            RED,
            RESET
        );
    }
}