use serde::{Deserialize, Serialize};

use crate::{
    message::MessageEntity,
    types::{Animation, Integer, PhotoSize, User},
};

/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub text_entities: Vec<Option<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

/// A placeholder, currently holds no information. Use BotFather to set up your game.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CallbackGame;

/// This object represents one row of the high scores table for a game.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    pub position: Integer,
    pub user: User,
    pub score: Integer,
}
