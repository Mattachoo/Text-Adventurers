use crate::accessible::{Accessible, Accessor};
use crate::stat::StatBlock;

pub struct Character {
    pub name: String,
    pub stats: StatBlock,
    accessor: Accessor<Character>,
}

impl Character {
    pub fn new(name: String, stats: StatBlock) -> Character {
        let mut accessor = Accessor::new();
        accessor.register("name", Box::new(|c: &Character| Some(c.name.clone())));
        Character {
            name,
            stats,
            accessor,
        }
    }
}

impl Accessible for Character {
    fn lookup_local(&self, property: &str) -> Option<String> {
        self.accessor.lookup(property, self)
    }
}
