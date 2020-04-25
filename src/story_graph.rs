use std::collections::HashMap;

use crate::io::Interface;
use crate::world::World;

pub struct StoryGraph {
    node_name_to_node: HashMap<String, StoryNode>,
}

impl StoryGraph {
    pub fn new() -> StoryGraph {
        StoryGraph {
            node_name_to_node: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: StoryNode) {
        self.node_name_to_node.insert(node.name.clone(), node);
    }

    pub fn get_node(&self, key: &str) -> &StoryNode {
        &self.node_name_to_node[key]
    }
}

pub struct StoryNode {
    name: String,
    elements: Vec<StoryElement>,
}

impl StoryNode {
    pub fn empty_node(name: String) -> StoryNode {
        StoryNode {
            name,
            elements: Vec::new(),
        }
    }

    pub fn new(name: String, elements: Vec<StoryElement>) -> StoryNode {
        StoryNode { name, elements }
    }
}

pub enum StoryElement {
    Text(StoryText),
    Choice(StoryChoice),
    Exit,
}

impl StoryElement {
    fn run<I: Interface>(
        &self,
        index: usize,
        current_node: &StoryNode,
        graph: &StoryGraph,
        interface: &mut I,
        world: World,
    ) {
        match self {
            StoryElement::Text(text) => text.run(index, current_node, graph, interface, world),
            StoryElement::Choice(choice) => {
                choice.run(index, current_node, graph, interface, world)
            }
            StoryElement::Exit => {
                interface.write("Goodbye! Thanks for playing.");
            }
        }
    }
}

pub struct StoryChoice {
    options: Vec<StoryOption>,
}

impl StoryChoice {
    fn run<I: Interface>(
        &self,
        index: usize,
        current_node: &StoryNode,
        graph: &StoryGraph,
        interface: &mut I,
        world: World,
    ) {
        // TODO(amclees): Implement this function.
    }
}

pub struct StoryOption {
    intro_text: String,
    result_text: Option<String>,
    next_node: Option<String>,
}

pub struct StoryText {
    text: String,
}

impl StoryText {
    fn run<I: Interface>(
        &self,
        index: usize,
        current_node: &StoryNode,
        graph: &StoryGraph,
        interface: &mut I,
        world: World,
    ) {
        interface.write(self.text.as_str());
        current_node.elements[index + 1].run(index, current_node, graph, interface, world);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::character::Character;
    use crate::io::TestInterface;
    use crate::stat::StatBlock;
    use crate::world::World;

    #[test]
    pub fn stores_nodes() {
        let mut graph = StoryGraph::new();
        graph.add_node(StoryNode::empty_node(String::from("FooNode")));
        assert_eq!(graph.get_node("FooNode").name, "FooNode");
    }

    #[test]
    pub fn text_nodes_write_text() {
        let node = StoryNode::new(
            "FooNode".to_string(),
            vec![
                StoryElement::Text(StoryText {
                    text: "Sample text.\n".to_string(),
                }),
                StoryElement::Exit,
            ],
        );
        let graph = StoryGraph::new();
        let mut interface = TestInterface::new(VecDeque::new());
        let world = World {
            player: Character {
                stats: StatBlock::new(),
            },
        };
        node.elements[0].run(0, &node, &graph, &mut interface, world);
        assert_eq!(
            interface.written,
            "Sample text.\nGoodbye! Thanks for playing."
        );
    }
}
