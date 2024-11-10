use crate::domain::models::enemy::{ArmoredEnemy, Enemy, EnemyBehavior};

pub struct EnemyFactory;

impl EnemyFactory {
    pub fn create_enemy(name: &str) -> Box<dyn EnemyBehavior> {
        Box::new(Enemy::new(name.to_string()))
    }

    pub fn create_armored_enemy(name: &str, armor: i32) -> Box<dyn EnemyBehavior> {
        Box::new(ArmoredEnemy::new(
            Enemy::new(name.to_string()),
            armor,
        ))
    }
}
