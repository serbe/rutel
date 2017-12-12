use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug)]
pub struct SendVideoParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet,
    /// or upload a new video using multipart/form-data. More info on Sending Files »
    pub video: File,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Video caption (may also be used when resending videos by file_id), 0-200 characters
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

impl SendVideoParams {
    pub fn new(chat_id: ChatID, video: File) -> Self {
        SendVideoParams {
            chat_id,
            video,
            duration: None,
            width: None,
            height: None,
            caption: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn duration(&mut self, v: Integer) -> &mut Self {
        self.duration = Some(v);
        self
    }

    pub fn width(&mut self, v: Integer) -> &mut Self {
        self.width = Some(v);
        self
    }

    pub fn height(&mut self, v: Integer) -> &mut Self {
        self.height = Some(v);
        self
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
