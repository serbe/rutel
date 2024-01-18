use serde::{Deserialize, Serialize};

use crate::{
    message::Message,
    types::{Boolean, Chat, Integer, User},
};

/// This object represents a service message about the creation of a scheduled giveaway. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GiveawayCreated {}

/// This object represents a message about a scheduled giveaway.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Giveaway {
    /// The list of chats which the user must join to participate in the giveaway
    pub chats: Vec<Chat>,
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    pub winners_selection_date: Integer,
    /// The number of users which are supposed to be selected as winners of the giveaway
    pub winner_count: Integer,
    /// Optional. True, if only users who join the chats after the giveaway started should be eligible to win
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<Boolean>,
    /// Optional. True, if the list of giveaway winners will be visible to everyone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<Boolean>,
    /// Optional. Description of additional giveaway prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
    /// Optional. A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    /// Optional. The number of months the Telegram Premium subscription won from the giveaway will be active for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<Integer>,
}

/// This object represents a message about the completion of a giveaway with public winners.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GiveawayWinners {
    /// The chat that created the giveaway
    pub chat: Chat,
    /// Identifier of the messsage with the giveaway in the chat
    pub giveaway_message_id: Integer,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    pub winners_selection_date: Integer,
    /// Total number of winners in the giveaway
    pub winner_count: Integer,
    /// List of up to 100 winners of the giveaway
    pub winners: Vec<User>,
    /// Optional. The number of other chats the user had to join in order to be eligible for the giveaway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<Integer>,
    /// Optional. The number of months the Telegram Premium subscription won from the giveaway will be active for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<Integer>,
    /// Optional. Number of undistributed prizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<Integer>,
    /// Optional. True, if only users who had joined the chats after the giveaway started were eligible to win
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<Boolean>,
    /// Optional. True, if the giveaway was canceled because the payment for it was refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<Boolean>,
    /// Optional. Description of additional giveaway prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}

/// This object represents a service message about the completion of a giveaway without public winners.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway
    pub winner_count: Integer,
    /// Optional. Number of undistributed prizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<Integer>,
    /// Optional. Message with the giveaway that was completed, if it wasn't deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<Message>>,
}
