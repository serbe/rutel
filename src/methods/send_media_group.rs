use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug, GetSet)]
pub struct SendMediaGroupParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    pub media: Vec<InputMedia>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom
    /// reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMediaGroupParams {
    pub fn new(chat_id: ChatID, media: Vec<InputMedia>) -> Self {
        SendMediaGroupParams {
            chat_id,
            media,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}
