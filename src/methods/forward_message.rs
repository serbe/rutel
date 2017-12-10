use types::*;
use serde_json::to_string;

/// Use this method to forward messages of any kind. On success, the sent Message is returned.
#[derive(Serialize, Debug)]
pub struct forwardMessageParams {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: ChatID,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: Integer,
}

impl forwardMessageParams {
    pub fn new(chat_id: ChatID, from_chat_id: ChatID, message_id: Integer) -> Self {
        forwardMessageParams {
            chat_id,
            from_chat_id,
            disable_notification: None,
            message_id,
        }
    }

    pub fn disable_notification(&mut self, v: Boolean) -> &mut Self {
        self.disable_notification = Some(v);
        self
    }

    pub fn json(&self) -> String {
        to_string(self).unwrap()
    }
}
