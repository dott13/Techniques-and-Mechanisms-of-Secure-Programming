use std::sync::Arc;

use crate::domain::models::enemy::EnemyBehavior;
use crate::domain::models::player::{Player, Item};

use super::game_event::{EventManager, GameEvent, PlayerObserver};

pub struct Game {
    players: Vec<Player>,
    enemies: Vec<Box<dyn EnemyBehavior>>,
}

impl Game {
    fn new() -> Game {
        // Broadcast game initialization event
        let event_manager = EventManager::instance();
        event_manager.lock().unwrap().broadcast_event(&GameEvent::GameStarted);
        
        Game {
            players: Vec::new(),
            enemies: Vec::new(),
        }
    }

    pub fn instance() -> &'static mut Game {
        static mut INSTANCE: Option<Game> = None;
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Game::new());
            }
            INSTANCE.as_mut().unwrap()
        }
    }

    pub fn add_player(&mut self, player: Player) {
        // Add an observer for this player
        let event_manager = EventManager::instance();
        let player_observer = Arc::new(PlayerObserver::new(player.name.clone()));
        event_manager.lock().unwrap().add_observer(player.name.clone(), player_observer);

        // Broadcast player joined event
        event_manager.lock().unwrap().broadcast_event(&&GameEvent::PlayerJoined(player.name.clone()));
        
        self.players.push(player);
    }

    pub fn add_enemy(&mut self, enemy: Box<dyn EnemyBehavior>) {
        // Broadcast enemy spawned event
        let event_manager = EventManager::instance();
        event_manager.lock().unwrap().broadcast_event(&GameEvent::EnemySpawned(enemy.get_name().to_string()));
        
        self.enemies.push(enemy);
    }

    // Restore the add_item_to_player method
    pub fn add_item_to_player(&mut self, player_name: &str, item: Item) {
        if let Some(player) = self.players.iter_mut().find(|p| p.name == player_name) {
            player.collect_item(&item.name);
        }
    }

    pub fn start_game(&mut self) {
        println!("Starting the game!");
        
        for player in &self.players {
            player.show_inventory();
        }
   
        // Combat simulation with event notifications
        for enemy in &mut self.enemies {
            println!("A wild {} appears!", enemy.get_name());
            
            for player in &mut self.players {
                println!("{} attacks {}!", player.name, enemy.get_name());
                
                // Simulate damage and notify observers
                let damage = 10;
                enemy.attack(damage);
                
                let event_manager = EventManager::instance();
                event_manager.lock().unwrap().broadcast_event(&GameEvent::PlayerDamaged(player.name.clone(), damage));
                
                if enemy.get_health() <= 0 {
                    println!("{} has been defeated!", enemy.get_name());
                    
                    // Notify observers about enemy defeat
                    let event_manager = EventManager::instance();
                    event_manager.lock().unwrap().broadcast_event(&GameEvent::EnemyDefeated(enemy.get_name().to_string()));
                    
                    break;
                }
            }
        }

        // Broadcast game end event
        let event_manager = EventManager::instance();
        event_manager.lock().unwrap().broadcast_event(&GameEvent::GameEnded);
    }
}