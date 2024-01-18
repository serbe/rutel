use serde::{Deserialize, Serialize};

use crate::{
    message::MessageEntity,
    types::{Boolean, Chat, Integer, User},
};

/// This object contains information about one answer option in a poll.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PollOption {
    pub text: String,
    pub voter_count: Integer,
}

/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// Optional. The chat that changed the answer to the poll, if the voter is anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,
    /// Optional. The user that changed the answer to the poll, if the voter isn't anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    pub option_ids: Vec<Integer>,
}

/// This object contains information about a poll.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-300 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: Integer,
    /// True, if the poll is closed
    pub is_closed: Boolean,
    /// True, if the poll is anonymous
    pub is_anonymous: Boolean,
    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub kind: String,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: Boolean,
    /// Optional. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<Integer>,
    /// Optional. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Optional. Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Optional. Amount of time in seconds the poll will be active after creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<Integer>,
    /// Optional. Point in time (Unix timestamp) when the poll will be automatically closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<Integer>,
}
