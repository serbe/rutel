use types::*;
use serde_json::to_string;

/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .mp3 format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
///
/// For sending voice messages, use the sendVoice method instead.
#[derive(Serialize, Debug)]
pub struct sendAudioParams {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub audio: File,
    /// Audio caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    // Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl sendAudioParams {
    pub fn new(chat_id: ChatID, audio: File) -> Self {
        sendAudioParams {
            chat_id,
            audio,
            caption: None,
            duration: None,
            performer: None,
            title: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn caption(&mut self, v: String) -> &mut Self {
        self.caption = Some(v);
        self
    }

    pub fn duration(&mut self, v: Integer) -> &mut Self {
        self.duration = Some(v);
        self
    }

    pub fn performer(&mut self, v: String) -> &mut Self {
        self.performer = Some(v);
        self
    }

    pub fn title(&mut self, v: String) -> &mut Self {
        self.title = Some(v);
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
