use crate::domain::models::enemy::Enemy;


pub struct EnemyFactory;

impl EnemyFactory {
    pub fn create_enemy(name: &str) -> Enemy {
        Enemy::new(name.to_string())
    }
}