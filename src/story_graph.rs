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
}

enum StoryElement {
    Text(StoryText),
    Choice(StoryChoice),
}

impl StoryElement {
    fn run<I: Interface>(
        &self,
        index: i64,
        current_node: &StoryNode,
        graph: &StoryGraph,
        interface: I,
        world: World,
    ) {
        match self {
            StoryElement::Text(text) => text.run(index, current_node, graph, interface, world),
            StoryElement::Choice(choice) => {
                choice.run(index, current_node, graph, interface, world)
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
        index: i64,
        current_node: &StoryNode,
        graph: &StoryGraph,
        interface: I,
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
        index: i64,
        current_node: &StoryNode,
        graph: &StoryGraph,
        interface: I,
        world: World,
    ) {
        // TODO(amclees): Implement this function.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn stores_nodes() {
        let mut graph = StoryGraph::new();
        graph.add_node(StoryNode::empty_node(String::from("FooNode")));
        assert_eq!(graph.get_node("FooNode").name, "FooNode");
    }
}
