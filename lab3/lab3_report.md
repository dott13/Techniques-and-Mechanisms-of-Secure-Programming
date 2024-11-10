# Structural Design Patterns

**Author:** Popov Tudor

## Introduction

This project aims to explore and implement Structural Design Patterns in Rust. And to get a mark :)

## Objectives

1. Study and understand the Structural Design Patterns.
2. As a continuation of the previous laboratory work, think about the functionalities that your system will need to provide to the user.
3. Implement some additional functionalities using structural design patterns.

## Main Files Added/Changed

These files are:

- **Enemy**: Changed it to work with the Decorator Structural Pattern.
- **Entities**: Added for Composition Pattern so we can create A Party of Players.
- **Game**: Changed it to work with our Facade Game System.
- **Game System**: Added for Facade Pattern.
- **Client**: Changed the main function to work with the new additions.

### Instantiation Methods

- **Facade**: This Pattern simplifies the interface for a complex system with multiple classes, interconections and etc by making it unified. It also acts as a wrapper, reduces dependencies.
- **Composition**: It allows creating complex object structures from small basis. Treats uniformly some individual objects that are of the same origin. Enables building tree-like hierarchical structures.
- **Decorator**:  Dynamically adds new behaviours to an object, provides a flexible alternative to subclass for extending functionality. And it follows open-closed principle by extending functionality without modifying existing code.

### Examples

- Facade: For facade we created a new structure called `GameSystem`, where we initialize our players equip them and do the same with our enemies and start the game using our `Game` structure.

```rust
pub fn initialize_game(&self, players: Vec<Player>, enemies: Vec<Box<dyn EnemyBehavior>>) {
        let game = Game::instance();
        for player in players {
            game.add_player(player);
        }
        for enemy in enemies {
            game.add_enemy(enemy);
        }
    }

    pub fn equip_players(&self, equipment_map: Vec<(&str, Item)>) {
        let game = Game::instance();
        for (player_name, item) in equipment_map {
            game.add_item_to_player(player_name, item);
        }
    }

    pub fn prepare_enemies(&self, enemies: Vec<Box<dyn EnemyBehavior>>) -> Vec<Box<dyn EnemyBehavior>> {
        enemies
    }

    pub fn run_game(&self) {
        let game = Game::instance();
        game.start_game();
    }
```

- Composition: For composition we needed to add the `Entities` structure and composing them with our `Player`. It works the same with our `Enemies`. We add those in an EntityGroup that will have the entities we added for future battles.

```rust
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
```

It uses the EntityComponent for developing the fully fledged Group.

```rust
impl EntityComponent for Player {
    fn display_status(&self) {
        println!("Player {} - Health: {}", self.name, self.health);
        self.show_inventory();
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}
```

- Decorator: For making this pattern we needed to edit our `Enemy` and `EnemyFactory` we added a new type of enemy called the `Armored Enemy` now it will work with any new **enemy types** we want to add. For it actually to work we made it so it doesnt work with Enemy it will now work with `EnemyBehaviour` so now all the enimies will have different traits.

```rust
pub struct ArmoredEnemy {
    enemy: Enemy,
    armor: i32,
}

impl ArmoredEnemy {
    pub fn new(enemy: Enemy, armor: i32) -> Self {
        ArmoredEnemy { enemy, armor }
    }
```

```rust
impl EnemyBehavior for ArmoredEnemy {
    fn attack(&mut self, damage: i32) {
        let reduced_damage = std::cmp::max(1, damage - self.armor); // Armor absorbs part of the damage
        self.enemy.attack(reduced_damage); // Delegate reduced damage to the base enemy
        println!("{}'s armor absorbed {} damage!", self.enemy.name, damage - reduced_damage);
    }
    

    fn get_name(&self) -> &str {
        &self.enemy.name
    }

    fn get_health(&self) -> i32 {
        self.enemy.health
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_base_enemy(&self) -> Enemy {
        self.enemy.clone()
    }
}
```

## Conclusion

In this project we implemented three new Design Patterns that do not change absolutely anything since if you are already working on a complex project you will work with those patterns automatically without thinking.
