use std::collections::HashMap;

pub struct Inventory {
    // Key is Item.id.
    items: HashMap<String, ItemStack>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items
            .entry(item.id.clone())
            .or_insert(ItemStack { item, count: 0 })
            .count += 1;
    }

    pub fn get_stack_by_id(&self, id: &str) -> Option<&ItemStack> {
        let stack_option = self.items.get(id);
        if let Some(stack) = stack_option {
            if stack.empty() {
                return None;
            }
        }
        stack_option
    }

    pub fn get_mut_stack_by_id(&mut self, id: &str) -> Option<&mut ItemStack> {
        let stack_option = self.items.get_mut(id);
        if let Some(stack) = &stack_option {
            if stack.empty() {
                return None;
            }
        }
        stack_option
    }

    pub fn total_item_count(&self) -> i64 {
        let mut count = 0;
        for (_id, stack) in &self.items {
            count += stack.count;
        }
        count
    }

    pub fn empty(&self) -> bool {
        self.total_item_count() == 0
    }
}

#[derive(Debug, PartialEq)]
pub struct ItemStack {
    item: Item,
    count: i64,
}

impl ItemStack {
    pub fn item(&self) -> &Item {
        &self.item
    }

    pub fn count(&self) -> i64 {
        self.count
    }

    pub fn add(&mut self) {
        self.count += 1;
    }

    // Adds n items to the stack. Negative values for n are a noop.
    pub fn add_n(&mut self, n: i64) {
        if n < 0 {
            return;
        }
        self.count += n
    }

    pub fn remove(&mut self) {
        if !self.empty() {
            self.count -= 1;
        }
    }

    pub fn remove_n(&mut self, n: i64) {
        if n < 0 {
            return;
        }
        if n > self.count {
            self.count = 0
        } else {
            self.count -= n;
        }
    }

    pub fn empty(&self) -> bool {
        self.count == 0
    }
}

#[derive(Debug, PartialEq)]
pub struct Item {
    id: String,
    name: String,
    behaviors: Vec<ItemBehavior>,
}

impl Item {
    pub fn new(id: String, name: String, behaviors: Vec<ItemBehavior>) -> Item {
        Item {
            id,
            name,
            behaviors,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn behaviors(&self) -> &[ItemBehavior] {
        &self.behaviors
    }
}

#[derive(Debug, PartialEq)]
pub enum ItemBehavior {
    Key(KeyData),
    // TODO(andrewmclees): Implement as other modules become available.
    Equipment,
}

#[derive(Debug, PartialEq)]
pub struct KeyData {
    doors: Vec<String>,
}

impl KeyData {
    pub fn new(doors: Vec<String>) -> KeyData {
        KeyData { doors }
    }

    pub fn opens(&self, door: &str) -> bool {
        for openable_door in &self.doors {
            if door == openable_door {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_inventories_start_empty() {
        assert_eq!(Inventory::new().total_item_count(), 0);
        assert!(Inventory::new().empty());
    }

    #[test]
    pub fn inventories_hold_items() {
        let mut inventory = Inventory::new();
        assert_eq!(inventory.get_stack_by_id("id1"), None);
        inventory.add_item(Item::new(
            String::from("id1"),
            String::from("Item 1"),
            vec![],
        ));
        assert!(!inventory.empty());
        assert_eq!(inventory.total_item_count(), 1);
        assert_eq!(
            inventory.get_stack_by_id("id1"),
            Some(&ItemStack {
                item: Item::new(String::from("id1"), String::from("Item 1"), vec![]),
                count: 1,
            }),
        );
        inventory.get_mut_stack_by_id("id1").unwrap().add();
        assert_eq!(inventory.get_stack_by_id("id1").unwrap().count(), 2);
        inventory.get_mut_stack_by_id("id1").unwrap().add_n(5);
        assert_eq!(inventory.get_stack_by_id("id1").unwrap().count(), 7);
        inventory.get_mut_stack_by_id("id1").unwrap().remove_n(4);
        assert_eq!(inventory.get_stack_by_id("id1").unwrap().count(), 3);
        inventory.get_mut_stack_by_id("id1").unwrap().remove();
        assert_eq!(inventory.get_stack_by_id("id1").unwrap().count(), 2);
        inventory.get_mut_stack_by_id("id1").unwrap().remove_n(300);
        assert_eq!(inventory.get_stack_by_id("id1"), None);
    }
}
