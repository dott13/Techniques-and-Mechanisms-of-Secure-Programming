use crate::domain::models::player::{Player, Item};

pub struct PlayerFactory;

impl PlayerFactory {
    pub fn create_player(name: &str) -> Player {
        Player::new(name.to_string())
    }
}

pub struct ItemFactory;

impl ItemFactory {
    pub fn create_item(name: &str) -> Item {
        Item::new(name.to_string())
    }
}