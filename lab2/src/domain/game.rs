use crate::domain::models::player::{Player, Item};
use crate::domain::models::enemy::Enemy;

pub struct Game {
    players: Vec<Player>,
    enemies: Vec<Enemy>,
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

    pub fn add_enemy(&mut self, enemy: Enemy) {
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
}

pub struct GameBuilder {
    players: Vec<Player>,
    enemies: Vec<Enemy>,
}

impl GameBuilder {
    pub fn new() -> Self {
        GameBuilder {
            players: Vec::new(),
            enemies: Vec::new(),
        }
    }

    pub fn add_player(mut self, player: Player) -> Self {
        self.players.push(player);
        self
    }

    pub fn add_enemy(mut self, enemy: Enemy) -> Self {
        self.enemies.push(enemy);
        self
    }

    pub fn build(self) {
        let game = Game::instance();
        for player in self.players {
            game.add_player(player);
        }
        for enemy in self.enemies {
            game.add_enemy(enemy);
        }
    }
}
