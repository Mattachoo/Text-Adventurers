use std::io;
use std::io::Write;

use crate::choice::Choice;

pub trait Interface {
    fn write(&mut self, message: &str);
    fn choose<T: Choice>(&mut self, choices: Vec<T>) -> T;
}

pub struct StandardIoInterface;

impl Interface for StandardIoInterface {
    fn write(&mut self, message: &str) {
        println!("{}", message);
    }

    fn choose<T: Choice>(&mut self, mut choices: Vec<T>) -> T {
        for (i, choice) in choices.iter().enumerate() {
            println!("{}) {}", i + 1, choice.describe());
        }
        println!("What do you do?");
        loop {
            print!("> ");
            io::stdout().flush().expect("failed to flush stdio");
            let mut chosen = String::new();
            io::stdin()
                .read_line(&mut chosen)
                .expect("failed to read from stdin");
            match chosen.trim().parse::<usize>() {
                Ok(chosen_index) => {
                    if chosen_index < 1 || chosen_index > choices.len() {
                        println!(
                            "Not a valid choice; choose a choice from 1 to {}.",
                            choices.len()
                        );
                        continue;
                    }
                    return choices.swap_remove(chosen_index - 1);
                }
                Err(_) => {
                    println!("Not a valid choice; enter a number.");
                }
            }
        }
    }
}
