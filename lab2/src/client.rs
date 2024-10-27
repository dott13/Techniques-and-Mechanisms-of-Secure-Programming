mod domain;

fn main() {
    let game = domain::game::Game::instance();
    
    // Create some players and items
    let player1 = domain::factory::player_factory::PlayerFactory::create_player("Hero");
    let player2 = domain::factory::player_factory::PlayerFactory::create_player("Sidekick");
    let sword = domain::factory::player_factory::ItemFactory::create_item("Sword");
    let shield = domain::factory::player_factory::ItemFactory::create_item("Shield");
    let enemy = domain::factory::enemy_factory::EnemyFactory::create_enemy("Goblin");

    // Add players and enemy to the game
    game.add_player(player1);
    game.add_player(player2);
    game.add_enemy(enemy);
    
    // Add items to players
    game.add_item_to_player("Hero", sword);
    game.add_item_to_player("Sidekick", shield);
    
    // Start the game loop
    game.start_game();
}
