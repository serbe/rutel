use serde::{Deserialize, Serialize};

use crate::types::{Boolean, Chat, Integer, User};

/// This object describes the source of a chat boost. It can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatBoostSource {
    ChatBoostSourcePremium(ChatBoostSourcePremium),
    ChatBoostSourceGiftCode(ChatBoostSourceGiftCode),
    ChatBoostSourceGiveaway(ChatBoostSourceGiveaway),
}

/// The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatBoostSourcePremium {
    /// Source of the boost, always “premium”
    pub source: String,
    /// User that boosted the chat
    pub user: User,
}

/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatBoostSourceGiftCode {
    /// Source of the boost, always “gift_code”
    pub source: String,
    /// User for which the gift code was created
    pub user: User,
}

///The boost was obtained by the creation of a Telegram Premium giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatBoostSourceGiveaway {
    /// Source of the boost, always “giveaway”
    pub source: String,
    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    pub giveaway_message_id: Integer,
    /// Optional. User that won the prize in the giveaway if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Optional. True, if the giveaway was completed, but there was no user to win the prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<Boolean>,
}

/// This object contains information about a chat boost.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatBoost {
    /// Unique identifier of the boost
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the chat was boosted
    pub add_date: Integer,
    /// Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
    pub expiration_date: Integer,
    /// Source of the added boost
    pub source: ChatBoostSource,
}

/// This object represents a boost added to a chat or changed.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted
    pub chat: Chat,
    /// Infomation about the chat boost
    pub boost: ChatBoost,
}

/// This object represents a boost removed from a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatBoostRemoved {
    /// Chat which was boosted
    pub chat: Chat,
    /// Unique identifier of the boost
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the boost was removed
    pub remove_date: Integer,
    /// Source of the removed boost
    pub source: ChatBoostSource,
}

/// This object represents a list of boosts added to a chat by a user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user
    pub boosts: Vec<ChatBoost>,
}
