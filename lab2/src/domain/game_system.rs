use super::{game::Game, models::{enemy::{ArmoredEnemy, Enemy, EnemyBehavior}, player::{Item, Player}}};

pub struct GameSystem;

impl GameSystem {
    pub fn new() -> Self {
        GameSystem
    }

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
}