use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug)]
pub struct SendPhotoParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet,
    /// or upload a new photo using multipart/form-data. More info on Sending Files »
    pub photo: FilePtr,
    /// Photo caption (may also be used when resending photos by file_id), 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
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

impl SendPhotoParams {
    pub fn new(chat_id: ChatID, photo: FilePtr) -> Self {
        SendPhotoParams {
            chat_id,
            photo,
            caption: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn caption(&mut self, v: String) -> &mut Self {
        self.caption = Some(v);
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
