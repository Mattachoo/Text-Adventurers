use std::io;
use std::io::Write;

pub trait Choice {
    fn describe(&self) -> String;
}

pub trait ConstantChoice {
    fn describe_str(&self) -> &str;
}

impl<T> Choice for T
where
    T: ConstantChoice,
{
    fn describe(&self) -> String {
        String::from(self.describe_str())
    }
}

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

pub struct World;

#[derive(Debug)]
enum Summit {
    Gaze,
    Descend,
}

impl ConstantChoice for Summit {
    fn describe_str(&self) -> &str {
        match self {
            Summit::Gaze => "Gaze down at the world below.",
            Summit::Descend => "Follow the narrow path.",
        }
    }
}

struct ExitMarker;

fn enter<I: Interface>(mut interface: I, world: World) -> ExitMarker {
    interface.write("Welcome to the world!");
    summit(interface, world)
}

fn summit<I: Interface>(mut interface: I, world: World) -> ExitMarker {
    interface.write(
        "
You find yourself upon the summit of a mountain towering over the landscape.
The summit is thin strip of smooth obsidian several hundred yards long and
a few feet tall.",
    );
    match interface.choose(vec![Summit::Gaze, Summit::Descend]) {
        Summit::Gaze => {
            interface.write(
                "
You gaze. To the east is a small port town, then sea as far as the
eye can see. In all other directions, the forest covering this mountain fade into
dense marshes. Past the marsh to the south, there are some woods and perhaps a road,
but they are obscured by fog.",
            );
            summit(interface, world)
        }
        Summit::Descend => {
            interface.write("You descend the narrow path. There is a cabin at the halfway point.");
            outside_cabin(interface, world)
        }
    }
}

enum OutsideCabin {
    GoInside,
    Ascend,
    Descend,
}

impl ConstantChoice for OutsideCabin {
    fn describe_str(&self) -> &str {
        match self {
            OutsideCabin::GoInside => "Try the cabin door.",
            OutsideCabin::Ascend => "Continue up the path to the summit of the mountain.",
            OutsideCabin::Descend => "Follow the path that leads down from the cabin.",
        }
    }
}

fn outside_cabin<I: Interface>(mut interface: I, world: World) -> ExitMarker {
    interface.write(
        "
The cabin is small but sturdy and well-maintained.",
    );
    match interface.choose(vec![
        OutsideCabin::GoInside,
        OutsideCabin::Ascend,
        OutsideCabin::Descend,
    ]) {
        OutsideCabin::GoInside => {
            interface.write("The door is unlocked, and you head inside.");
            exit(interface, world)
        }
        OutsideCabin::Ascend => summit(interface, world),
        OutsideCabin::Descend => {
            interface.write("You head down the path");
            exit(interface, world)
        }
    }
}

fn exit<I: Interface>(mut interface: I, _world: World) -> ExitMarker {
    interface.write("Goodbye! Thanks for playing.");
    ExitMarker
}

fn main() {
    let interface = StandardIoInterface {};
    let world = World {};
    enter(interface, world);
}