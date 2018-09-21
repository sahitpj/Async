extern crate libc;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::Command;

pub mod colors;
pub mod shell;
pub mod utils;



fn main() {
    command_line();
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
        libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    }
}

fn command_line() {
        let mut last_exit_status = true;
        utils::print_prompt(last_exit_status);
        let mut command_string = String::new();
        io::stdout().flush().expect("Failed to flush the Buffer");
        io::stdin()
            .read_line(&mut command_string) // reads user input
            .expect("Failed to read the user command");
        command_string.pop();
        // removes empty line
        if command_string.is_empty() == true {
            command_line();
        } else {
            while command_string.chars().last() == Some('\\') {
            command_string.pop();
            //println!("{}",command_string );
            let mut next_string = String::new();
            io::stdin()
                .read_line(&mut next_string)
                .expect("Failed to read the next line");
            next_string.pop();
            command_string.push_str(&next_string);
        }
        let commands = tokenize_commands(&mut command_string);
        for mut command in commands {
            for mut dependent_command in command {
                let mut is_background = false;
                if let Some(&"&") = dependent_command.last() {
                    is_background = true;
                    dependent_command.pop();
                }
                match dependent_command[0] {
                    "quit" => std::process::exit(0),
                    "cd" => {
                        last_exit_status = shell::change_dir(dependent_command[1]);
                    }
                    _ => {
                        last_exit_status = execute_command(dependent_command, is_background);
                    }
                }
                if last_exit_status {
                command_line();
                }
            }
        }
    }
}



fn tokenize_commands(command_string: &mut String) -> Vec<Vec<Vec<&str>>> {
    let commands: Vec<&str> = command_string.split(';').collect();
    let mut command_tokens: Vec<Vec<Vec<&str>>> = Vec::new();
    for command in commands.iter() {
        let mut dependent_commands: Vec<&str> = command.split("&&").collect();
        let mut temp_vec: Vec<Vec<&str>> = Vec::new();
        for dependent_command in dependent_commands.iter() {
            temp_vec.push(dependent_command.split_whitespace().collect());
        }
        command_tokens.push(temp_vec);
    }
    command_tokens
}

fn execute_command(command_tokens: Vec<&str>, is_background: bool) -> bool {
    let mut command_instance = Command::new(command_tokens[0]);
    if let Ok(mut child) = command_instance
        .args(&command_tokens[1..])
        .before_exec(|| {
            unsafe {
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
            }
            Result::Ok(())
        })
        .spawn()
    {
        if is_background == false {
            return child.wait().expect("command wasn't running").success();
        } else {
            colors::success_logger(format!("{} started!", child.id()));
            true
        }
    } else {
        shell::error_logger("Command not found!".to_string());
        true
    }
}




