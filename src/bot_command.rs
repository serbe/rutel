use serde::{Deserialize, Serialize};

use crate::types::{ChatID, Integer};

/// This object represents a bot command.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BotCommandScope {
    BotCommandScopeDefault,
    BotCommandScopeAllPrivateChats,
    BotCommandScopeAllGroupChats,
    BotCommandScopeAllChatAdministrators,
    BotCommandScopeChat,
    BotCommandScopeChatAdministrators,
    BotCommandScopeChatMember,
}

/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeDefault {
    /// Scope type, must be default
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents the scope of bot commands, covering all private chats.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeAllPrivateChats {
    /// Scope type, must be all_private_chats
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents the scope of bot commands, covering all group and supergroup chats.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeAllGroupChats {
    /// Scope type, must be all_group_chats
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeAllChatAdministrators {
    /// Scope type, must be all_chat_administrators
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents the scope of bot commands, covering a specific chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeChat {
    /// Scope type, must be chat
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Represents the scope of bot commands, covering all administrators of a specific group or supergroup chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeChatAdministrators {
    /// Scope type, must be chat_administrators
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}
