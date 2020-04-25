use std::collections::HashMap;

use crate::choice::Choice;
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
        let chosen = interface.choose(self.options.clone());
        if let Some(result_text) = chosen.result_text {
            interface.write(result_text.as_str());
        }
        if let Some(next_node) = chosen.next_node {
            let next_node = graph.get_node(next_node.as_str());
            next_node.elements[0].run(0, next_node, graph, interface, world);
        } else {
            current_node.elements[index + 1].run(index + 1, current_node, graph, interface, world);
        }
    }
}

#[derive(Clone)]
pub struct StoryOption {
    intro_text: String,
    result_text: Option<String>,
    next_node: Option<String>,
}

impl Choice for StoryOption {
    fn describe(&self) -> String {
        self.intro_text.clone()
    }
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
    use crate::io::TestInterface;
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
        let world = World::empty();
        node.elements[0].run(0, &node, &graph, &mut interface, world);
        assert_eq!(
            interface.written,
            "Sample text.\nGoodbye! Thanks for playing."
        );
    }

    #[test]
    pub fn choices_without_a_next_node_run() {
        let node = StoryNode::new(
            "FooNode".to_string(),
            vec![
                StoryElement::Choice(StoryChoice {
                    options: vec![
                        StoryOption {
                            intro_text: "Foo\n".to_string(),
                            result_text: None,
                            next_node: None,
                        },
                        StoryOption {
                            intro_text: "Bar\n".to_string(),
                            result_text: Some("Baz\n".to_string()),
                            next_node: None,
                        },
                    ],
                }),
                StoryElement::Exit,
            ],
        );
        let graph = StoryGraph::new();
        let mut interface = TestInterface::new(VecDeque::from(vec![1, 0]));
        let world = World::empty();
        node.elements[0].run(0, &node, &graph, &mut interface, world);
        assert_eq!(interface.written, "Baz\nGoodbye! Thanks for playing.");
        interface.written.clear();
        let world = World::empty();
        node.elements[0].run(0, &node, &graph, &mut interface, world);
        assert_eq!(interface.written, "Goodbye! Thanks for playing.");
    }

    #[test]
    pub fn choices_with_a_next_node_run() {
        let mut graph = StoryGraph::new();
        graph.add_node(StoryNode::new(
            "FooNode".to_string(),
            vec![
                StoryElement::Text(StoryText {
                    text: "Choose Foo or Bar\n".to_string(),
                }),
                StoryElement::Choice(StoryChoice {
                    options: vec![
                        StoryOption {
                            intro_text: "Foo\n".to_string(),
                            result_text: Some("Chose Foo\n".to_string()),
                            next_node: Some("FooNode".to_string()),
                        },
                        StoryOption {
                            intro_text: "Bar\n".to_string(),
                            result_text: None,
                            next_node: Some("BarNode".to_string()),
                        },
                    ],
                }),
            ],
        ));
        graph.add_node(StoryNode::new(
            "BarNode".to_string(),
            vec![
                StoryElement::Text(StoryText {
                    text: "Reached Bar\n".to_string(),
                }),
                StoryElement::Exit,
            ],
        ));
        let mut interface = TestInterface::new(VecDeque::from(vec![0, 0, 0, 1]));
        let world = World::empty();
        let node = graph.get_node("FooNode");
        node.elements[0].run(0, node, &graph, &mut interface, world);
        assert_eq!(
            interface.written,
            "Choose Foo or Bar
Chose Foo
Choose Foo or Bar
Chose Foo
Choose Foo or Bar
Chose Foo
Choose Foo or Bar
Reached Bar
Goodbye! Thanks for playing."
        );
    }
}
