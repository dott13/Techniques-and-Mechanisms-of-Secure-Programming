# Behavioral Design Patterns

**Author:** Popov Tudor

## Introduction

This project extends the game system we created for this project/laboratory by implementing the Observer Behavioral Design Pattern, adding events and the observer itself.

## Objectives

1. Study and understand the Behavioral Design Patterns.
2. As a continuation of the previous laboratory work, think about what communication between software entities might be involed in your system.
3. Implement some additional functionalities using behavioral design patterns.

## Main Files Added/Changed

- **EventManager**: New singleton class managing event observers
- **GameEvent**: Enum defining various game events
- **Observer**: Trait for objects that can receive game events
- **Game**: Modified to integrate with the event notification system

### Observer Pattern

The Observer Pattern allows objects to be notified automatically about changes or events in the system without tight coupling, it makes tracking the progress of the project and adds ease to managing the functionalities of it. Key characteristics include:

- Defines a one-to-many dependency between objects
- When one object changes state, all its dependents are notified and updated automatically
- Promotes loose coupling between the subject and observers

### Implementation Details

#### Game Events

The `GameEvent` enum captures various game-related events:
```rust
pub enum GameEvent {
    PlayerJoined(String),
    EnemySpawned(String),
    PlayerDamaged(String, i32),
    EnemyDefeated(String),
    GameStarted,
    GameEnded,
}
```

#### Event Manager

A singleton `EventManager` manages observers and event broadcasting:
```rust
pub struct EventManager {
    observers: Mutex<HashMap<String, Arc<dyn Observer>>>,
}
```

#### Observer Implementation

The `PlayerObserver` demonstrates how game entities can react to events:
```rust
impl Observer for PlayerObserver {
    fn update(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player_name) => {
                println!("{} noticed that {} joined the game", self.name, player_name);
            },
            // ... other event handlers
        }
    }
}
```

## Conclusion

In this update of our project we made managing the console logs and new events and functions appearing in our game have more relevancy. I don't know for what we did that because its the last laboratory work.
