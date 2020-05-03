use crate::accessible::{AccessPath, Accessible};

pub enum Token {
    Text(String),
    Accessor(AccessPath),
}

struct Template {
    pub tokens: Vec<Token>,
}

impl Template {
    // Builds a plaintext template from a &str (no parsing).
    pub fn raw_from_str(text: &str) -> Template {
        Template {
            tokens: vec![Token::Text(String::from(text))],
        }
    }

    pub fn render(&self, accessible: &Accessible) -> String {
        let mut rendered = String::new();
        for token in &self.tokens {
            match token {
                Token::Text(string) => rendered.push_str(&string),
                Token::Accessor(path) => {
                    if let Some(value) = accessible.lookup(path.view()) {
                        rendered.push_str(&value);
                    }
                }
            }
        }
        rendered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessible::{AccessPath, Accessor};

    struct TestContainer {
        a: String,
        accessor: Accessor<TestContainer>,
    }

    impl TestContainer {
        fn new() -> TestContainer {
            let mut container = TestContainer {
                a: String::from("foo"),
                accessor: Accessor::new(),
            };
            container.accessor.register(
                "a",
                Box::new(|container: &TestContainer| Some(container.a.clone())),
            );
            container
        }
    }

    impl Accessible for TestContainer {
        fn lookup_local(&self, property: &str) -> Option<String> {
            self.accessor.lookup(property, self)
        }
    }

    #[test]
    fn renders_text() {
        let template = Template {
            tokens: vec![
                Token::Text(String::from("Foo\n")),
                Token::Text(String::from("Bar Baz.")),
            ],
        };
        let container = TestContainer::new();
        assert_eq!(template.render(&container), "Foo\nBar Baz.");
    }

    #[test]
    fn renders_raw_text() {
        let template = Template::raw_from_str("Foo Bar");
        let container = TestContainer::new();
        assert_eq!(template.render(&container), "Foo Bar");
    }

    #[test]
    fn renders_accessed_value() {
        let template = Template {
            tokens: vec![
                Token::Text(String::from("Value: (")),
                Token::Accessor(AccessPath::from(String::from("a"))),
                Token::Text(String::from(")")),
            ],
        };
        let container = TestContainer::new();
        assert_eq!(template.render(&container), "Value: (foo)");
    }
}
