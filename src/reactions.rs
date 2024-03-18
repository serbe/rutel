use serde::{Deserialize, Serialize};

use crate::types::{Chat, Integer, User};

/// This object describes the type of a reaction. Currently, it can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ReactionType {
    ReactionTypeEmoji(ReactionTypeEmoji),
    ReactionTypeCustomEmoji(ReactionTypeCustomEmoji),
}

/// The reaction is based on an emoji.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReactionTypeEmoji {
    /// Type of the reaction, always “emoji”
    #[serde(rename = "type")]
    pub kind: String,
    /// Reaction emoji. Currently, it can be one of "👍", "👎", "❤", "🔥", "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩", "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳", "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨", "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻", "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡", "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘", "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
    pub emoji: String,
}

/// The reaction is based on a custom emoji.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReactionTypeCustomEmoji {
    /// Type of the reaction, always “custom_emoji”
    #[serde(rename = "type")]
    pub kind: String,
    /// Custom emoji identifier
    pub custom_emoji_id: String,
}

/// Represents a reaction added to a message along with the number of times it was added.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReactionCount {
    /// Type of the reaction
    #[serde(rename = "type")]
    pub kind: ReactionType,
    /// Number of times the reaction was added
    pub total_count: Integer,
}

/// This object represents a change of a reaction on a message performed by a user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to
    pub chat: Chat,
    /// Unique identifier of the message inside the chat
    pub message_id: Integer,
    /// Optional. The user that changed the reaction, if the user isn't anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Optional. The chat on behalf of which the reaction was changed, if the user is anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
    /// Date of the change in Unix time
    pub date: Integer,
    /// Previous list of reaction types that were set by the user
    pub old_reaction: Vec<ReactionType>,
    /// New list of reaction types that have been set by the user
    pub new_reaction: Vec<ReactionType>,
}

/// This object represents reaction changes on a message with anonymous reactions.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: Integer,
    /// Date of the change in Unix time
    pub date: Integer,
    /// List of reactions that are present on the message
    pub reactions: Vec<ReactionCount>,
}
