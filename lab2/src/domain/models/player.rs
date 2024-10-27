pub struct Player {
    pub name: String,
    pub health: i32,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            health: 100,
            inventory: Vec::new(),
        }
    }

    // Change to accept a reference to a String and clone it
    pub fn collect_item(&mut self, item: &String) {
        self.inventory.push(item.clone()); // Clone the item to store it in the inventory
        println!("{} collected a {}!", self.name, item);
    }

    pub fn show_inventory(&self) {
        println!("{}'s Inventory: {:?}", self.name, self.inventory);
    }
}

pub struct Item {
    pub name: String,
}

impl Item {
    pub fn new(name: String) -> Item {
        Item { name }
    }
}
