use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug, Default)]
pub struct StopMessageLiveLocationParams {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatID>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom
    /// reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocationParams {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn chat_id(&mut self, v: ChatID) -> &mut Self {
        self.chat_id = Some(v);
        self
    }

    pub fn message_id(&mut self, v: Integer) -> &mut Self {
        self.message_id = Some(v);
        self
    }

    pub fn inline_message_id(&mut self, v: String) -> &mut Self {
        self.inline_message_id = Some(v);
        self
    }

    pub fn reply_markup(&mut self, v: InlineKeyboardMarkup) -> &mut Self {
        self.reply_markup = Some(v);
        self
    }

    pub fn json(&self) -> String {
        to_string(self).unwrap()
    }
}
