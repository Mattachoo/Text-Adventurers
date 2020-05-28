use crate::adventure_parser;
use rayon::iter::split;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc::*;

//rules of the syntax:
//Some character to signify the start of a choice
//Narrator is the default character. It is reset to narrator at the start of each choice. A different character can be set by surrounding their name with back ticks: `
//Each choice ends at an empty new line, and then waits for user input.
//Depending on user input, a specified choice is called.
//My Initial idea is to store choices in a map, so that you can easily switch between them. Aa grid like structure could be made to make a more logical structure, in which only potential choices are connected to the current one.
//We need to define a character to signify an operation/function call.

pub fn run() {
    let filename = "C:\\Users\\Mattachu\\Documents\\text-adventurers\\src\\sample_input_0.txt";

    let story_graph_text: &str = &*read_to_string(filename).expect("File not Found");
    // Can use crossbeam-utils here as well for scoped thread
    let (sender, receiver) = channel();
    Lexer::lex(story_graph_text, sender);
    while let Ok(token) = receiver.recv() {
        println!("Token received from channel: {:?}", token);
    }
}
#[derive(Debug)]
pub enum Token<'a> {
    Title(&'a str, usize),
    Action(&'a str, Vec<&'a str>, usize),
    Text(&'a str, usize),
    Check(&'a bool, usize),
}
//Enum of states, which determine what operations are run
pub enum State {
    Title,
    Text,
    Action,
    GoTo,
    Check,
    Next,
    Proceed,
    End,
    Error,
}
pub struct Lexer<'a> {
    input: &'a str,
    start: usize,
    line: usize,
    pos: usize,
    width: usize,
    token_sender: Sender<Token<'a>>,
    current_line: usize,
}
struct StateFunction(fn(&mut Lexer) -> Option<StateFunction>);

impl<'a> Lexer<'a> {
    pub fn lex(s: &'a str, sender: Sender<Token<'a>>) {
        let mut lexer = Lexer::<'a> {
            input: s,
            start: 0,
            pos: 0,
            width: 0,
            token_sender: sender,
            current_line: 0,
            line: 0,
        };
        lexer.run();
    }
    fn run(&mut self) {
        //println!("Reached");
        let mut state = self.process();
        loop {
            //println!("Position: {}", self.pos);
            match state {
                State::End => break,
                State::Title => state = self.lex_title(),
                State::Next => state = self.next(),
                State::GoTo => state = self.lex_goto(),
                State::Proceed => state = self.process(),
                _ => state = self.lex_text(),
            }
        }
    }

    fn next(&mut self) -> State {
        if self.pos >= self.input.len() {
            //exit from processing, end of input reached
            State::End
        } else {
            //Update current line, reset positon
            //Move to end of line, and set start to the next line
            self.current_line += 1;
            self.pos += 1;
            self.start = self.pos;
            State::Proceed
        }
    }
    fn emit(&mut self, token: Token<'a>) {
        println!("Sending token on channel: {:?}", token);
        self.token_sender
            .send(token)
            .expect("Unable to send token on channel");
        //        self.start = self.pos;
    }

    fn process(&mut self) -> State {
        let mut state: State = State::End;
        /* println!(
            "{}",
            &self.input[self.start..self.getNextPosOfChar(self.start, '\n')]
        );*/
        if self.eof() {
            println!("EOF");
            state = State::End;
        } else if self.peek().trim() == "----" {
            state = State::Title;
        } else if (&self.input[self.start..self.get_next_pos_of_char(self.start, '\n')])
            .contains("->")
        {
            state = State::GoTo;
        } else {
            state = State::Text;
        }
        state
    }

    //Function to process titles
    fn lex_title(&mut self) -> State {
        //Peek next line, if it is '----', then current line is a title
        //Send the title to the parser
        // println!("Lex Title: ");
        self.pos = self.get_next_pos_of_char(self.start, '\n');
        self.emit(Token::Title(
            &self.input[self.start..self.pos],
            self.current_line,
        ));
        self.pos = self.get_next_pos_of_char(self.pos, '\n');
        State::Next
    }
    //Function to process ->
    fn lex_goto(&mut self) -> State {
        // println!("Lex GoTo: ");

        //if the line contains a '->', then the line contains an action
        //create token which contains the function name and a vector of its arguements
        let line_break: usize = self.get_next_pos_of_char(self.pos, '\n');
        let line = &self.input[self.start..line_break];
        let test: Vec<&str> = line.split("->").collect();
        let parameter = vec![test[1]];
        self.emit(Token::Action("GoTo", parameter, self.current_line));
        self.pos = line_break;

        State::Next
    }
    //function to process lines which start with ?
    fn lex_check(&mut self) {
        //TODO: Matthew Gerlits
        //Compares a given value, with the value of the given property

    }
    fn lex_marker(lex: &mut Lexer) {
        //TODO: Matthew Gerlits
        //If the line starts with '?', then it is a check
        //Create a token containing the type of check, and the name of the value it is checking, as well as the value it is checking against. If no value to check against exist, then it assumes it is a boolean check

    }
    fn lex_text(&mut self) -> State {
        //  println!("Lex Text: ");

        //Send the text to the parser
        self.pos = self.get_next_pos_of_char(self.start, '\n');
        self.emit(Token::Text(
            &self.input[self.start..self.pos],
            self.current_line,
        ));
        self.start = self.pos;
        State::Next
        //Then return some state
    }
    fn lex_narrator(lex: &mut Lexer) {
        //TODO: Matthew Gerlits
        //Create and send a token with signifies a change in narrator

    }
    fn eof(&self) -> bool {
        self.pos > self.input.len() - 1
    }
    //For our purposes, we need some version of "determine_token" that checks if a new Node is starting?
    //Return the next line
    fn peek(&self) -> &str {
        let i: usize = self.get_next_pos_of_char(self.start, '\n') + 1;
        let j: usize = self.get_next_pos_of_char(i, '\n');
        if self.eof() {
            //Something stating that EOF reached
        }
        &self.input[i..j]
    }
    fn validate() {}
    fn get_next_pos_of_char(&self, currentPos: usize, check: char) -> usize {
        let mut newPos: usize = 0;
        while self.input.len() > currentPos + newPos
            && self.input[(currentPos + newPos)..].chars().next().unwrap() != check
        {
            newPos += 1;
        }
        currentPos + newPos
    }
    fn is_action_word() {
        //check to see if the string contains keywords
    }
}
