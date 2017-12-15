use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug, GetSet)]
pub struct ForwardMessageParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier for the chat where the original message was sent (or channel username in
    /// the format @channelusername)
    pub from_chat_id: ChatID,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: Integer,
}

impl ForwardMessageParams {
    pub fn new(chat_id: ChatID, from_chat_id: ChatID, message_id: Integer) -> Self {
        ForwardMessageParams {
            chat_id,
            from_chat_id,
            disable_notification: None,
            message_id,
        }
    }
}
