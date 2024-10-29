mod domain;

fn main() {
    // Use GameBuilder to set up the game instance
    let mut builder = domain::game::GameBuilder::new();
    
    // Create some players and items
    let player1 = domain::factory::player_factory::PlayerFactory::create_player("Hero");
    let player2 = domain::factory::player_factory::PlayerFactory::create_player("Sidekick");
    let sword = domain::factory::player_factory::ItemFactory::create_item("Sword");
    let shield = domain::factory::player_factory::ItemFactory::create_item("Shield");
    let enemy = domain::factory::enemy_factory::EnemyFactory::create_enemy("Goblin");

    // Add players and enemy to the builder
    builder = builder.add_player(player1).add_player(player2).add_enemy(enemy);
    
    // Build the game (adds players and enemies to the singleton instance)
    builder.build();
    
    // Get the singleton game instance
    let game = domain::game::Game::instance();

    // Add items to players
    game.add_item_to_player("Hero", sword);
    game.add_item_to_player("Sidekick", shield);
    
    // Start the game loop
    game.start_game();
}
