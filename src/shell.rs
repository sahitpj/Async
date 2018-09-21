
use std::path::Path;
use std::env;

use colors::RESET;
use colors::RED;


pub fn change_dir(new_path: &str) -> bool {
    let new_path = Path::new(new_path);
    match env::set_current_dir(&new_path) {
        Err(err) => {
            error_logger(format!("Failed to change the directory!\n{}", err));
            return false;
        }
        _ => (),
    }
    return true;
}

pub fn error_logger(string: String) {
    println!("{}{}{}", RED, string, RESET);
}