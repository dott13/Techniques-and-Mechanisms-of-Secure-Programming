use crate::domain::models::player::{Player, Item};

use super::models::enemy::EnemyBehavior;

pub struct Game {
    players: Vec<Player>,
    enemies: Vec<Box<dyn EnemyBehavior>>,
}

impl Game {
    fn new() -> Game {
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
        self.players.push(player);
    }

    pub fn add_enemy(&mut self, enemy: Box<dyn EnemyBehavior>) {
        self.enemies.push(enemy);
    }    

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
    
        // Combat simulation
        for enemy in &mut self.enemies {
            println!("A wild {} appears!", enemy.get_name());
            for player in &mut self.players {
                println!("{} attacks {}!", player.name, enemy.get_name());
                enemy.attack(10); // This should dynamically call the correct implementation
                if enemy.get_health() <= 0 {
                    println!("{} has been defeated!", enemy.get_name());
                    break;
                }
            }
        }
    }    
}