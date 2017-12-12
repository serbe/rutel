use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug)]
pub struct SendMessageParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Text of the message to be sent
    pub text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or
    /// inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<Boolean>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessageParams {
    pub fn new(chat_id: ChatID, text: String) -> Self {
        SendMessageParams {
            chat_id,
            text,
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn parse_mode(&mut self, v: String) -> &mut Self {
        self.parse_mode = Some(v);
        self
    }

    pub fn disable_web_page_preview(&mut self, v: Boolean) -> &mut Self {
        self.disable_web_page_preview = Some(v);
        self
    }

    pub fn disable_notification(&mut self, v: Boolean) -> &mut Self {
        self.disable_notification = Some(v);
        self
    }

    pub fn reply_to_message_id(&mut self, v: Integer) -> &mut Self {
        self.reply_to_message_id = Some(v);
        self
    }

    pub fn reply_markup(&mut self, v: ReplyMarkup) -> &mut Self {
        self.reply_markup = Some(v);
        self
    }

    pub fn json(&self) -> String {
        to_string(self).unwrap()
    }
}
