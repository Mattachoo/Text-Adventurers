use std::collections::HashMap;

pub type AccessFunction<T> = Box<dyn Fn(&T) -> Option<String>>;

pub struct Accessor<T> {
    properties: HashMap<String, AccessFunction<T>>,
}

impl<T> Accessor<T> {
    pub fn new() -> Accessor<T> {
        Accessor {
            properties: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: &str, function: AccessFunction<T>) {
        self.properties.insert(key.to_owned(), function);
    }

    pub fn lookup(&self, key: &str, accessible: &T) -> Option<String> {
        let access_function = self.properties.get(key)?;
        access_function(accessible)
    }
}

#[derive(Clone)]
pub struct AccessPath {
    segments: Vec<String>,
}

impl AccessPath {
    pub fn from(string: String) -> AccessPath {
        let mut path = AccessPath {
            segments: Vec::new(),
        };
        for segment in string.split(".") {
            path.segments.push(segment.to_owned());
        }
        path
    }

    pub fn view(&self) -> AccessPathView {
        AccessPathView {
            segments: &self.segments[..],
        }
    }
}

pub struct AccessPathView<'a> {
    segments: &'a [String],
}

pub trait Accessible {
    fn lookup(&self, path: AccessPathView) -> Option<String> {
        if path.segments.len() == 0 {
            return None;
        }
        if path.segments.len() == 1 {
            return self.lookup_local(&path.segments[0]);
        }
        self.get_child(&path.segments[0])?.lookup(AccessPathView {
            segments: &path.segments[1..],
        })
    }
    fn lookup_local(&self, property: &str) -> Option<String>;
    fn get_child(&self, _child: &str) -> Option<&Accessible> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContainer {
        a: String,
        b: String,
        child: TestSubcontainer,
        accessor: Accessor<TestContainer>,
    }

    impl Accessible for TestContainer {
        fn lookup_local(&self, property: &str) -> Option<String> {
            self.accessor.lookup(property, self)
        }
        fn get_child(&self, child: &str) -> Option<&Accessible> {
            match child {
                "child" => Some(&self.child),
                _ => None,
            }
        }
    }

    struct TestSubcontainer {
        c: String,
        accessor: Accessor<TestSubcontainer>,
    }

    impl Accessible for TestSubcontainer {
        fn lookup_local(&self, property: &str) -> Option<String> {
            self.accessor.lookup(property, self)
        }
    }

    #[test]
    fn looks_up_properties() {
        let mut container = TestContainer {
            a: String::from("foo"),
            b: String::from("bar"),
            child: TestSubcontainer {
                c: String::from("baz"),
                accessor: Accessor::new(),
            },
            accessor: Accessor::new(),
        };
        container.accessor.register(
            "a",
            Box::new(|container: &TestContainer| Some(container.a.clone())),
        );
        container.accessor.register(
            "b",
            Box::new(|container: &TestContainer| Some(container.b.clone())),
        );
        container.child.accessor.register(
            "c",
            Box::new(|container: &TestSubcontainer| Some(container.c.clone())),
        );
        assert_eq!(
            container.accessor.lookup("a", &container),
            Some(String::from("foo"))
        );
        assert_eq!(
            container.lookup(AccessPath::from(String::from("a")).view()),
            Some(String::from("foo"))
        );
        assert_eq!(
            container.lookup(AccessPath::from(String::from("b")).view()),
            Some(String::from("bar"))
        );
        assert_eq!(
            container.lookup(AccessPath::from(String::from("child.c")).view()),
            Some(String::from("baz"))
        );
        assert_eq!(
            container.lookup(AccessPath::from(String::from("")).view()),
            None
        );
        assert_eq!(
            container.lookup(AccessPath::from(String::from("c")).view()),
            None
        );
        assert_eq!(
            container.lookup(AccessPath::from(String::from("d")).view()),
            None
        );
        assert_eq!(
            container.lookup(AccessPath::from(String::from("child.c.foo")).view()),
            None
        );
    }
}
