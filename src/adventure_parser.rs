use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::io;

//rules of the syntax:
//some character to signify the start of a choice
//Narrator is the default character. It is reset to narrator at the start of each choice. A different character can be set by surrounding their name with back ticks: `
//trailing '----' makes parsing a bit harder	
//each choice ends at an empty new line, and waits for user input
//depending on user input, a choice is called
//initial idea is to store choices in a map, so that you can easily switch between them. Aa grid like structure could be made to make a more logical structure, in which only potential choices are connected to the current one
//need a character to signify an operation/function call

struct Choice{
	title: String,
	dialogue: String,
	option: String,
}

const TITLE : char = '#';
const SPEAKER : char = '`';
const ACTION : char = '`';
                
fn main()  -> Result<(), io::Error> {   
	let mut my_map = HashMap::new();
    let f = File::open("sample_input_0.txt")?;
    let file = BufReader::new(&f);
	let mut my_choice: Choice;
	//let mut current : String = "".to_string().to_owned();
	//let mut narrator : String = "".to_string().to_owned();
	let mut dialog : String= "".to_string().to_owned();
    for line in file.lines() {
		if let Ok(ip) = line {
			let mut current : String = "".to_string().to_owned();
			let mut narrator : String = "".to_string().to_owned();
			// let l = line..as_red().unwrap();
			if ip.chars().count() <= 0 {
				my_map.insert( 	current.clone(), Choice { title: current.to_string(), dialogue: dialog.to_string(), option: "".to_string() });
				narrator = "".to_string();
			} else {

				//character_check(l, current,my_map);
				let character = ip.chars( ).next().unwrap();
				match character {
					TITLE =>
					//add code to remove #
					current = ip,
				  SPEAKER =>
					//add code to remove backticks
					narrator = ip,
				_ =>
					dialog.push_str(&narrator),

			}
			}
			for (key, value) in &my_map {

				println!("{}", key);
			};
		}

    }
	Ok(())
}   

