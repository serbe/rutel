use serde::{Deserialize, Serialize};

use crate::{
    stickers::Sticker,
    types::{Boolean, Chat, Integer, Location, User},
};

/// Describes the connection of the bot with a business account.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection
    pub id: String,
    /// Business account user that created the business connection
    pub user: User,
    /// Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub user_chat_id: Integer,
    /// Date the connection was established in Unix time
    pub date: Integer,
    /// True, if the bot can act on behalf of the business account in chats that were active in the last 24 hours
    pub can_reply: Boolean,
    /// True, if the connection is active
    pub is_enabled: Boolean,
}

/// This object is received when messages are deleted from a connected business account.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
    pub chat: Chat,
    /// The list of identifiers of deleted messages in the chat of the business account
    pub message_ids: Vec<Integer>,
}

/// Contains information about the start page settings of a Telegram Business account.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BusinessIntro {
    /// Optional. Title text of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Message text of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional. Sticker of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
}

/// Contains information about the location of a Telegram Business account.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BusinessLocation {
    /// Address of the business
    pub address: String,
    /// Optional. Location of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// Describes an interval of time during which a business is open.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BusinessOpeningHoursInterval {
    /// The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 * 24 * 60
    pub opening_minute: Integer,
    /// The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 * 24 * 60
    pub closing_minute: Integer,
}

/// Describes the opening hours of a business.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined
    pub time_zone_name: String,
    /// List of time intervals describing business opening hours
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
