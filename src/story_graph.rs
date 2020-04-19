use std::collections::HashMap;

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
    elements: Vec<Box<StoryElement>>,
}

impl StoryNode {
    pub fn empty_node(name: String) -> StoryNode {
        StoryNode {
            name,
            elements: Vec::new(),
        }
    }
}

pub trait StoryElement {
    // TODO(andrewmclees): Add the IO interface and world as parameters.
    fn run(&self, index: i64, current_node: &StoryNode, graph: &StoryGraph);
}

pub struct StoryChoice {
    options: Vec<StoryOption>,
}

impl StoryElement for StoryChoice {
    fn run(&self, index: i64, current_node: &StoryNode, graph: &StoryGraph) {
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

impl StoryElement for StoryText {
    fn run(&self, index: i64, current_node: &StoryNode, graph: &StoryGraph) {
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
