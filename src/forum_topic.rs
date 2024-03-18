use serde::{Deserialize, Serialize};

use crate::types::Integer;

/// This object represents a service message about a new forum topic created in the chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: Integer,
    /// Optional. Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopicClosed {}

/// This object represents a service message about an edited forum topic.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopicEdited {
    /// Optional. New name of the topic, if it was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopicReopened {}

/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GeneralForumTopicHidden {}

/// This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GeneralForumTopicUnhidden {}
