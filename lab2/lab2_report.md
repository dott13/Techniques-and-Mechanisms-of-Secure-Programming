# Creational Design Patterns

**Author:** Popov Tudor

## Introduction

This project aims to explore and implement Creational Design Patterns in Rust, focusing on effective object creation mechanisms to address common design challenges in software engineering.

## Objectives

1. Study and understand Creational Design Patterns.
2. Choose a domain, define its main classes/models/entities, and choose the appropriate instantiation mechanisms.
3. Use some creational design patterns for object instantiation in a sample project.

## Domain Selection

For this project, I chose the domain of a simple game. The game includes players, enemies, and items, providing a clear context for applying various creational design patterns.

## Main Classes and Instantiation Mechanisms

The main classes in the project are:

- **Player**: Represents the player in the game.
- **Enemy**: Represents the enemies encountered in the game.
- **Item**: Represents items that players can collect.
- **Game**: Manages the game logic, including players and enemies.
- **Factories**: Classes that create instances of players, enemies, and items using specific patterns.

### Instantiation Mechanisms

- **Factory Method**: Used to create instances of `Player`, `Enemy`, and `Item` through dedicated factory classes (`PlayerFactory`, `EnemyFactory`).
- **Singleton**: Ensured that only one instance of the `Game` class exists throughout the application.

## Implemented Creational Design Patterns

In this project, I implemented the following creational design patterns:

1. **Factory Method**: Allows the creation of different objects (`Player`, `Enemy`) without specifying the exact class of the object being created.
2. **Singleton**: Ensures that there is a single instance of the `Game` class, providing a global point of access.

### Examples

- Factory Method: The strucutre is simple we have different factories for the enemies and the player with his items.

```rust
use crate::domain::models::enemy::Enemy;


pub struct EnemyFactory;

impl EnemyFactory {
    pub fn create_enemy(name: &str) -> Enemy {
        Enemy::new(name.to_string())
    }
}
```

And from the player_factory.rs:

```rust
use crate::domain::models::player::{Player, Item};

pub struct PlayerFactory;

impl PlayerFactory {
    pub fn create_player(name: &str) -> Player {
        Player::new(name.to_string())
    }
}

pub struct ItemFactory;

impl ItemFactory {
    pub fn create_item(name: &str) -> Item {
        Item::new(name.to_string())
    }
}
```

- Singleton: The game itself is called only once, the instance is made only once and is destroyed after for not having memory loss.

```rust
pub fn start_game(&mut self) {
        println!("Starting the game!");

        for player in &self.players {
            player.show_inventory();
        }

        // Simple combat simulation
        for enemy in &mut self.enemies {
            println!("A wild {} appears!", enemy.name);
            for player in &mut self.players {
                println!("{} attacks {}!", player.name, enemy.name);
                enemy.attack(10); // Simulate an attack
                if enemy.health <= 0 {
                    println!("{} has been defeated!", enemy.name);
                    break;
                }
            }
        }
    }
```

## Conclusion

This project successfully demonstrates the application of Creational Design Patterns in a game context, providing insight into effective object creation strategies. The implementation of the Factory Method and Singleton patterns simplified the creation and management of game entities while adhering to principles of modularity and maintainability.
