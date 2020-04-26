use std::collections::VecDeque;
use std::io;
use std::io::Write;
use std::iter::FromIterator;

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

pub struct TestInterface {
    pub written: String,
    pub preset_choices: VecDeque<usize>,
}

impl TestInterface {
    fn new(preset_choices: VecDeque<usize>) -> TestInterface {
        TestInterface {
            written: String::new(),
            preset_choices,
        }
    }
}

impl Interface for TestInterface {
    fn write(&mut self, message: &str) {
        self.written.push_str(message);
    }

    fn choose<T: Choice>(&mut self, mut choices: Vec<T>) -> T {
        choices.swap_remove(self.preset_choices.pop_front().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_interface_writes_messages() {
        let mut interface = TestInterface::new(VecDeque::new());
        assert_eq!(interface.written, String::new());
        interface.write("foo");
        assert_eq!(interface.written, "foo");
        interface.write(" bar");
        assert_eq!(interface.written, "foo bar");
    }

    #[derive(Clone)]
    struct TestChoice {
        val: i32,
    }

    impl Choice for TestChoice {
        fn describe(&self) -> String {
            self.val.to_string()
        }
    }

    #[test]
    pub fn test_interface_chooses_preset_options() {
        let mut interface = TestInterface::new(VecDeque::from(vec![0, 1, 0, 2]));
        let options = vec![
            TestChoice { val: 12 },
            TestChoice { val: 42 },
            TestChoice { val: 123 },
        ];
        assert_eq!(interface.choose(options.clone()).val, 12);
        assert_eq!(interface.choose(options.clone()).val, 42);
        assert_eq!(interface.choose(options.clone()).val, 12);
        assert_eq!(interface.choose(options.clone()).val, 123);
    }
}
