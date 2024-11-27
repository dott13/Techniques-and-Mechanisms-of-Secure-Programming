use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Define possible game events
#[derive(Clone, Debug)]
pub enum GameEvent {
    PlayerJoined(String),
    EnemySpawned(String),
    PlayerDamaged(String, i32),
    EnemyDefeated(String),
    GameStarted,
    GameEnded,
}

// Observer trait that defines how observers receive events
pub trait Observer: Send + Sync {
    fn update(&self, event: &GameEvent);
}

// Subject trait for objects that can be observed
pub trait Subject {
    fn add_observer(&mut self, observer: Arc<dyn Observer>);
    fn remove_observer(&mut self, observer_name: &str);
    fn notify_observers(&self, event: &GameEvent);
}

// Implement a central event manager using the Singleton pattern
pub struct EventManager {
    observers: Mutex<HashMap<String, Arc<dyn Observer>>>,
}

impl EventManager {
    fn new() -> Self {
        EventManager {
            observers: Mutex::new(HashMap::new()),
        }
    }

    // Singleton instance method
    pub fn instance() -> &'static Mutex<EventManager> {
        static mut INSTANCE: Option<Mutex<EventManager>> = None;
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Mutex::new(EventManager::new()));
            }
            INSTANCE.as_ref().unwrap()
        }
    }

    // Add an observer
    pub fn add_observer(&self, name: String, observer: Arc<dyn Observer>) {
        let mut observers = self.observers.lock().unwrap();
        observers.insert(name, observer);
    }

    // Remove an observer
    pub fn remove_observer(&self, name: &str) {
        let mut observers = self.observers.lock().unwrap();
        observers.remove(name);
    }

    // Broadcast an event to all observers
    pub fn broadcast_event(&self, event: &GameEvent) {
        let observers = self.observers.lock().unwrap();
        for observer in observers.values() {
            observer.update(event);
        }
    }
}

// Example Observer implementations
pub struct PlayerObserver {
    name: String,
}

impl PlayerObserver {
    pub fn new(name: String) -> Self {
        PlayerObserver { name }
    }
}

impl Observer for PlayerObserver {
    fn update(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player_name) => {
                if player_name != &self.name {
                    println!("{} noticed that {} joined the game", self.name, player_name);
                }
            },
            GameEvent::EnemySpawned(enemy_name) => {
                println!("{} noticed that {} has spawned!", self.name, enemy_name);
            },
            GameEvent::PlayerDamaged(player_name, damage) => {
                if player_name == &self.name {
                    println!("{} took {} damage!", self.name, damage);
                }
            },
            GameEvent::EnemyDefeated(enemy_name) => {
                println!("{} heard that {} was defeated!", self.name, enemy_name);
            },
            GameEvent::GameStarted => {
                println!("{} is ready for the game to begin!", self.name);
            },
            GameEvent::GameEnded => {
                println!("{} acknowledges the game has ended.", self.name);
            },
        }
    }
}