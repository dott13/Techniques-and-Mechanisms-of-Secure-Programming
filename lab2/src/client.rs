use domain::{factory::{enemy_factory::EnemyFactory, player_factory::{ItemFactory, PlayerFactory}}, game_system::GameSystem, utils::entities::EntityGroup};

mod domain;

fn main() {
    // Create a game system FACADE
    let game_system = GameSystem::new();

    let player1 = PlayerFactory::create_player("Hero");
    let player2 = PlayerFactory::create_player("Sidekick");
    
    let basic_enemy = EnemyFactory::create_enemy("Goblin");
    let armored_enemy = EnemyFactory::create_armored_enemy("Armored Troll", 5);
    println!(
        "{} has {} health and {} armor.",
        armored_enemy.get_name(),
        armored_enemy.get_health(),
        5
    );

    let sword = ItemFactory::create_item("Sword");
    let shield = ItemFactory::create_item("Shield");

    // Create an entity group using the COMPOSITE pattern
    let mut party = EntityGroup::new("Main Party".to_string());
    // Add the players to the entity group
    party.add_entity(Box::new(player1.clone()));
    party.add_entity(Box::new(player2.clone()));

    // Display initial party status
    party.display_group_status();

    let prepared_enemies = game_system.prepare_enemies(vec![
        basic_enemy,
        armored_enemy,
    ]);

    game_system.initialize_game(
        vec![player1.clone(), player2.clone()],
        prepared_enemies
    );

    // Equip players using the facade
    game_system.equip_players(vec![
        ("Hero", sword),
        ("Sidekick", shield),
    ]);

    // Start the game
    game_system.run_game();
}