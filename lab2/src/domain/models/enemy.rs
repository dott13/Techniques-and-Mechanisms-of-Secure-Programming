#[derive(Clone)]
pub struct Enemy {
    pub name: String,
    pub health: i32,
}

impl Enemy {
    pub fn new(name: String) -> Enemy {
        Enemy { name, health: 20 }
    }

    pub fn attack(&mut self, damage: i32) {
        self.health -= damage;
        println!("{} took {} damage! Health remaining: {}", self.name, damage, self.health);
    }
}
//Decorator
pub trait EnemyBehavior {
    fn attack(&mut self, damage: i32);
    fn get_name(&self) -> &str;
    fn get_health(&self) -> i32;
    fn as_any(&self) -> &dyn std::any::Any;
    fn get_base_enemy(&self) -> Enemy;
}

// Implement for base Enemy
impl EnemyBehavior for Enemy {
    fn attack(&mut self, damage: i32) {
        self.health -= damage;
        println!("{} took {} damage! Health remaining: {}", self.name, damage, self.health);
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_base_enemy(&self) -> Enemy {
        self.clone()
    }
}
pub struct ArmoredEnemy {
    enemy: Enemy,
    armor: i32,
}

impl ArmoredEnemy {
    pub fn new(enemy: Enemy, armor: i32) -> Self {
        ArmoredEnemy { enemy, armor }
    }

    // Add a method to get the base enemy
    pub fn into_enemy(self) -> Enemy {
        self.enemy
    }

    // Add a method to get a reference to the base enemy
    pub fn get_base_enemy(&self) -> &Enemy {
        &self.enemy
    }
}

impl EnemyBehavior for ArmoredEnemy {
    fn attack(&mut self, damage: i32) {
        let reduced_damage = std::cmp::max(1, damage - self.armor); // Armor absorbs part of the damage
        self.enemy.attack(reduced_damage); // Delegate reduced damage to the base enemy
        println!("{}'s armor absorbed {} damage!", self.enemy.name, damage - reduced_damage);
    }
    

    fn get_name(&self) -> &str {
        &self.enemy.name
    }

    fn get_health(&self) -> i32 {
        self.enemy.health
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_base_enemy(&self) -> Enemy {
        self.enemy.clone()
    }
}

impl Into<Enemy> for ArmoredEnemy {
    fn into(self) -> Enemy {
        self.enemy
    }
}
