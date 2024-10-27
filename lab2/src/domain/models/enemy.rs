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