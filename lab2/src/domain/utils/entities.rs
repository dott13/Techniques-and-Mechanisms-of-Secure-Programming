use crate::domain::models::{enemy::Enemy, player::Player};

pub struct EntityGroup {
    name: String,
    entities: Vec<Box<dyn EntityComponent>>,
}

pub trait EntityComponent {
    fn display_status(&self);
    fn is_alive(&self) -> bool;
}

impl EntityGroup {
    pub fn new(name: String) -> Self {
        EntityGroup {
            name,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn EntityComponent>) {
        self.entities.push(entity);
    }

    pub fn display_group_status(&self) {
        println!("Group '{}' status:", self.name);
        for entity in &self.entities {
            entity.display_status();
        }
    }
}

impl EntityComponent for Player {
    fn display_status(&self) {
        println!("Player {} - Health: {}", self.name, self.health);
        self.show_inventory();
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl EntityComponent for Enemy {
    fn display_status(&self) {
        println!("Enemy {} - Health: {}", self.name, self.health);
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}