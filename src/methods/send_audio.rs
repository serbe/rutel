use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug, GetSet)]
pub struct SendAudioParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio
    /// file from the Internet, or upload a new one using multipart/form-data. More info on Sending
    /// Files »
    pub audio: FilePtr,
    /// Audio caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Duration of the audio in seconds
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
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendAudioParams {
    pub fn new(chat_id: ChatID, audio: FilePtr) -> Self {
        SendAudioParams {
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
}
