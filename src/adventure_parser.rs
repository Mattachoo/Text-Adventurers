use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

//rules of the syntax:
//Some character to signify the start of a choice
//Narrator is the default character. It is reset to narrator at the start of each choice. A different character can be set by surrounding their name with back ticks: `
//Each choice ends at an empty new line, and then waits for user input.
//Depending on user input, a specified choice is called.
//My Initial idea is to store choices in a map, so that you can easily switch between them. Aa grid like structure could be made to make a more logical structure, in which only potential choices are connected to the current one.
//We need to define a character to signify an operation/function call.

struct Choice {
    title: String,
    dialogue: String,
    option: String,
}

const TITLE_TOKEN: char = '#';
const SPEAKER_TOKEN: char = '`';
const ACTION_TOKEN: char = '`';

fn build_map(input_file : & str) -> Result<(), io::Error> {
    let mut choices = HashMap::new();
    let f = File::open(input_file)?;
    let file = BufReader::new(&f);
    let mut my_choice: Choice;
    let mut dialog: String = "".to_string().to_owned();
    for line in file.lines() {
        if let Ok(ip) = line {
            let mut current: String = "".to_string().to_owned();
            let mut narrator: String = "".to_string().to_owned();
            // let l = line..as_red().unwrap();
            if ip.chars().count() <= 0 {
                choices.insert(
                    current.clone(),
                    Choice {
                        title: current.to_string(),
                        dialogue: dialog.to_string(),
                        option: "".to_string(),
                    },
                );
                narrator = "".to_string();
            } else {
                let character = ip.chars().next().unwrap();
                match character {
                    TITLE_TOKEN =>
                    //TODO(Mattachoo): add code to remove #
                    {
                        current = ip
                    }
                    SPEAKER_TOKEN =>
                    //TODO(Mattachoo):add code to remove backticks
                    {
                        narrator = ip
                    }
                    _ => dialog.push_str(&narrator),
                }
            }
            for (key, value) in &choices {
                println!("{}", key);
            }
        }
    }
    Ok(())
}
