/// sendDocument
/// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
///
/// Parameters	Type	Required	Description
/// chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
/// document	InputFile or String	Yes	File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
/// caption	String	Optional	Document caption (may also be used when resending documents by file_id), 0-200 characters
/// disable_notification	Boolean	Optional	Sends the message silently. Users will receive a notification with no sound.
/// reply_to_message_id	Integer	Optional	If the message is a reply, ID of the original message
/// reply_markup	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply	Optional	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.

use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug)]
pub struct sendDocumentParams {
    pub chat_id: ChatID,
    pub document: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl sendDocumentParams {
    pub fn new(chat_id: ChatID, document: File) -> Self {
        sendDocumentParams {
            chat_id,
            document,
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
