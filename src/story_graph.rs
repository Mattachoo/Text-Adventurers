use std::collections::HashMap;

use crate::choice::Choice;
use crate::io::Interface;
use crate::template::Template;
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
        let rendered_options = self.render_options(&world);
        let chosen_rendered = interface.choose(rendered_options);
        let chosen = &self.options[chosen_rendered.index];
        if let Some(result_text) = &chosen.result_text {
            interface.write(&result_text.render(&world));
        }
        if let Some(next_node) = &chosen.next_node {
            let next_node = graph.get_node(next_node.as_str());
            next_node.elements[0].run(0, next_node, graph, interface, world);
        } else {
            current_node.elements[index + 1].run(index + 1, current_node, graph, interface, world);
        }
    }

    fn render_options(&self, world: &World) -> Vec<RenderedStoryOption> {
        let mut rendered = Vec::new();
        for (index, option) in self.options.iter().enumerate() {
            rendered.push(RenderedStoryOption {
                rendered_intro: option.intro_text.render(world),
                index,
            });
        }
        rendered
    }
}

#[derive(Clone)]
pub struct StoryOption {
    intro_text: Template,
    result_text: Option<Template>,
    next_node: Option<String>,
}

struct RenderedStoryOption {
    rendered_intro: String,
    index: usize,
}

impl Choice for RenderedStoryOption {
    fn describe(&self) -> String {
        self.rendered_intro.clone()
    }
}

pub struct StoryText {
    text: Template,
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
        interface.write(&self.text.render(&world));
        current_node.elements[index + 1].run(index, current_node, graph, interface, world);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::accessible::AccessPath;
    use crate::io::TestInterface;
    use crate::template;
    use crate::template::Template;
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
                    text: Template {
                        tokens: vec![
                            template::Token::Text(String::from("player.name: ")),
                            template::Token::Accessor(AccessPath::from(String::from(
                                "player.name",
                            ))),
                            template::Token::Text(String::from(".\n")),
                        ],
                    },
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
            "player.name: Player.\nGoodbye! Thanks for playing."
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
                            intro_text: Template::raw_from_str("Foo\n"),
                            result_text: None,
                            next_node: None,
                        },
                        StoryOption {
                            intro_text: Template::raw_from_str("Bar\n"),
                            result_text: Some(Template::raw_from_str("Baz\n")),
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
                    text: Template::raw_from_str("Choose Foo or Bar\n"),
                }),
                StoryElement::Choice(StoryChoice {
                    options: vec![
                        StoryOption {
                            intro_text: Template::raw_from_str("Foo\n"),
                            result_text: Some(Template::raw_from_str("Chose Foo\n")),
                            next_node: Some(String::from("FooNode")),
                        },
                        StoryOption {
                            intro_text: Template::raw_from_str("Bar\n"),
                            result_text: None,
                            next_node: Some(String::from("BarNode")),
                        },
                    ],
                }),
            ],
        ));
        graph.add_node(StoryNode::new(
            "BarNode".to_string(),
            vec![
                StoryElement::Text(StoryText {
                    text: Template::raw_from_str("Reached Bar\n"),
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
