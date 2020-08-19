// use types::{Message, Response, Update, User};
use dotenv;
use rpc::client::Client;
use rutel_derive::Response;
use serde::Serialize;
use serde_json::{from_slice, from_value, to_string, Value};

use crate::error::{Error, Result};
use crate::types::*;

#[derive(Debug)]
pub struct Bot {
    pub token: String,
    pub proxy: Option<String>,
    pub user: Option<User>,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        let proxy = dotenv::var("PROXY").ok();
        Bot {
            token: token.to_string(),
            proxy: proxy,
            user: None,
        }
    }

    pub fn build_uri(&self, method: &'static str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }

    pub async fn create_request(&mut self, method: &'static str, values: String) -> Result<Value> {
        let uri = self.build_uri(method);

        dbg!(&uri);
        dbg!(&values);

        let client_builder = if let Some(proxy) = &self.proxy {
            Client::builder().proxy(&proxy)
        } else {
            Client::builder()
        };

        let mut client = client_builder
            .post(&uri)
            .body(values.as_bytes())
            .header("Content-Type", "application/json")
            .build()
            .await?;
        let _response = client.send().await?;
        let body = client.text().await?;
        let response = body.as_bytes().to_vec();

        let v: Value = from_slice(&response)?;
        let r: Response = from_value(v)?;
        if r.ok {
            let res: Value = r.result.ok_or(Error::NoResult)?;
            Ok(res)
        } else {
            let description = r.description.ok_or(Error::NoDescription)?;
            Err(Error::Description(description.to_string()))
        }
    }

    /// A simple method for testing your bot's auth token. Requires no parameters. Returns basic
    /// information about the bot in form of a User object.
    pub async fn get_me(&mut self) -> Result<User> {
        let resp = self.create_request("getMe", String::new()).await?;
        let user: User = from_value(resp)?;
        self.user = Some(user.clone());
        Ok(user)
    }

    // async fn post_json(&mut self, target: &str, body: &str) -> Result<Vec<u8>> {
    //     let mut client = Client::builder()
    //         .post(target)
    //         .body(body.as_bytes())
    //         .header("Content-Type", "application/json")
    //         .build()
    //         .await?;
    //     let _response = client.send().await?;
    //     let body = client.text().await?;
    //     Ok(body.as_bytes().to_vec())
    // }
}

// // setwebhook
// // deleteWebhook
// // getWebhookInfo

/// Use this method to receive incoming updates using long polling (wiki). An Array of Update
/// objects is returned.
#[response = "Vec<Update>"]
#[derive(Serialize, Debug, Response)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one than the highest
    /// among the identifiers of previously received updates. By default, updates starting with the
    /// earliest unconfirmed update are returned. An update is considered confirmed as soon as
    /// getUpdates is called with an offset higher than its update_id. The negative offset can be
    /// specified to retrieve updates starting from -offset update from the end of the updates
    /// queue. All previous updates will forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Integer>,
    /// Limits the number of updates to be retrieved. Values between 1—100 are accepted. Defaults
    /// to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be
    /// positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Integer>,
    /// List the types of updates you want your bot to receive. For example, specify [“message”,
    /// “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update
    /// for a complete list of available update types. Specify an empty list to receive all updates
    /// regardless of type (default). If not specified, the previous setting will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to send text messages. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendMessage {
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

/// Use this method to forward messages of any kind. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct ForwardMessage {
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

/// Use this method to send photos. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet,
    /// or upload a new photo using multipart/form-data. More info on Sending Files »
    pub photo: InputFileString,
    /// Photo caption (may also be used when resending photos by file_id), 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
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

/// Use this method to send audio files, if you want Telegram clients to display them in the music
/// player. Your audio must be in the .mp3 format. On success, the sent Message is returned. Bots
/// can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
///
/// For sending voice messages, use the sendVoice method instead.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendAudio {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio
    /// file from the Internet, or upload a new one using multipart/form-data. More info on Sending
    /// Files »
    pub audio: InputFileString,
    /// Audio caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using
    /// multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you
    /// can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
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

/// Use this method to send general files. On success, the sent Message is returned. Bots can currently
/// send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendDocument {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data. More info on Sending Files »
    pub document: InputFileString,
    /// Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using
    /// multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you
    /// can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
    /// Document caption (may also be used when resending documents by file_id), 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
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

/// Use this method to send video files, Telegram clients support mp4 videos (other formats may be
/// sent as Document). On success, the sent Message is returned. Bots can currently send video files
/// of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendVideo {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet,
    /// or upload a new video using multipart/form-data. More info on Sending Files »
    pub video: InputFileString,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using
    /// multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you
    /// can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
    /// Video caption (may also be used when resending videos by file_id), 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Pass True, if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<Boolean>,
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

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On
/// success, the sent Message is returned. Bots can currently send animation files of up to
/// 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendAnimation {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Animation to send. Pass a file_id as String to send an animation that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an
    /// animation from the Internet, or upload a new animation using multipart/form-data. More
    /// info on Sending Files »
    pub animation: InputFileString,
    /// Duration of sent animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB
    /// in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is
    /// not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail
    /// was uploaded using multipart/form-data under <file_attach_name>. More info on Sending
    /// Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
    /// Animation caption (may also be used when resending animation by file_id), 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text
    /// or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
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

/// Use this method to send audio files, if you want Telegram clients to display the file as a
/// playable voice message. For this to work, your audio must be in an .ogg file encoded with
/// OPUS (other formats may be sent as Audio or Document). On success, the sent Message is
/// returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be
/// changed in the future.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendVoice {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the
    /// Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub voice: InputFileString,
    /// Voice message caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text
    /// or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
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

/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long. Use
/// this method to send video messages. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendVideoNote {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Video note to send. Pass a file_id as String to send a video note that exists on the
    /// Telegram servers (recommended) or upload a new video using multipart/form-data. More info
    /// on Sending Files ». Sending video notes by a URL is currently unsupported
    pub video_note: InputFileString,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Video width and height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Integer>,
    /// Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB
    /// in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is
    /// not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail
    /// was uploaded using multipart/form-data under <file_attach_name>. More info on Sending
    /// Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
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

/// Use this method to send a group of photos or videos as an album. On success, an array of the
/// sent Messages is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<Message>"]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    pub media: Vec<InputMedia>,
    /// Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the messages are a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
}

/// Use this method to send point on the map. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Latitude of the location
    pub latitude: Float,
    /// Longitude of the location
    pub longitude: Float,
    /// Period in seconds for which the location will be updated (see Live Locations, should be
    /// between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
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

/// Use this method to edit live location messages sent by the bot or via the bot (for inline bots).
/// A location can be edited until its live_period expires or editing is explicitly disabled by a
/// call to stopMessageLiveLocation. On success, if the edited message was sent by the bot, the
/// edited Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageLiveLocation {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: Float,
    /// Longitude of new location
    pub longitude: Float,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to stop updating a live location message sent by the bot or via the bot (for
/// inline bots) before live_period expires. On success, if the message was sent by the bot, the
/// sent Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct StopMessageLiveLocation {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Latitude of the venue
    pub latitude: Float,
    /// Longitude of the venue
    pub longitude: Float,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
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

/// Use this method to send phone contacts. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendContact {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send a native poll. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Poll question, 1-255 characters
    pub question: String,
    /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the poll needs to be anonymous, defaults to True
    pub is_anonymous: Option<Boolean>,
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<Boolean>,
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<Integer>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll,
    /// 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<Integer>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5
    /// and no more than 600 seconds in the future. Can't be used together with open_period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<Integer>,
    /// Pass True, if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<Boolean>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard,
    /// instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendDice {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, or “🏀”.
    /// Dice can have values 1-6 for “🎲” and “🎯”, and values 1-5 for “🏀”. Defaults to “🎲”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// 	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard,
    /// instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method when you need to tell the user that something is happening on the bot's side.
/// The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients
/// clear its typing status). Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SendChatAction {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive:
    /// typing for text messages, upload_photo for photos, record_video or upload_video for videos,
    /// record_audio or upload_audio for audio files, upload_document for general files,
    /// find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: String,
}

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Serialize, Debug, Response)]
#[response = "UserProfilePhotos"]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Integer>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults
    /// to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>,
}

/// Use this method to get basic info about a file and prepare it for downloading. For the moment,
/// bots can download files of up to 20MB in size. On success, a File object is returned. The file
/// can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where
/// <file_path> is taken from the response. It is guaranteed that the link will be valid for at
/// least 1 hour. When the link expires, a new one can be requested by calling getFile again.
#[derive(Serialize, Debug, Response)]
#[response = "File"]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

/// Use this method to kick a user from a group, a supergroup or a channel. In the case of supergroups
/// and channels, the user will not be able to return to the group on their own using invite links,
/// etc., unless unbanned first. The bot must be an administrator in the chat for this to work and
/// must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct KickChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or
    /// less than 30 seconds from the current time they are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Integer>,
}

/// Use this method to unban a previously kicked user in a supergroup or channel. The user will not
/// return to the group or channel automatically, but will be able to join via link, etc. The bot
/// must be an administrator for this to work. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in
    /// the format @username)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the
/// supergroup for this to work and must have the appropriate admin rights. Pass True for all
/// boolean parameters to lift restrictions from a user. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for
    /// more than 366 days or less than 30 seconds from the current time, they are considered to be
    /// restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Integer>,
}

/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an
/// administrator in the chat for this to work and must have the appropriate admin rights. Pass
/// False for all boolean parameters to demote a user. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Pass True, if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<Boolean>,
    /// Pass True, if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<Boolean>,
    /// Pass True, if the administrator can edit messages of other users and can pin messages,
    /// channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<Boolean>,
    /// Pass True, if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<Boolean>,
    /// Pass True, if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<Boolean>,
    /// Pass True, if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<Boolean>,
    /// Pass True, if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
    /// Pass True, if the administrator can add new administrators with a subset of his own
    /// privileges or demote administrators that he has promoted, directly or indirectly (promoted
    /// by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<Boolean>,
}

/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

/// Use this method to set default chat permissions for all members. The bot must be an administrator
/// in the group or a supergroup for this to work and must have the can_restrict_members admin rights.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatPermissions {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// New default chat permissions
    pub permissions: ChatPermissions,
}

/// Use this method to generate a new invite link for a chat; any previously generated link is
/// revoked. The bot must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Returns the new invite link as String on success.
#[derive(Serialize, Debug, Response)]
#[response = "String"]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to set a new profile photo for the chat. Photos can't be changed for private
/// chats. The bot must be an administrator in the chat for this to work and must have the appropriate
/// admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}

/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must
/// be an administrator in the chat for this to work and must have the appropriate admin rights.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to change the title of a chat. Titles can't be changed for private chats.
/// The bot must be an administrator in the chat for this to work and must have the appropriate
/// admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatTitle {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// New chat title, 1-255 characters
    pub title: String,
}

/// Use this method to change the description of a supergroup or a channel. The bot must be an
/// administrator in the chat for this to work and must have the appropriate admin rights.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Use this method to pin a message in a supergroup or a channel. The bot must be an
/// administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right
/// in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct PinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Identifier of a message to pin
    pub message_id: Integer,
    /// Pass True, if it is not necessary to send a notification to all chat members about the new
    /// pinned message. Notifications are always disabled in channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
}

/// Use this method to unpin a message in a supergroup or a channel. The bot must be an
/// administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right
/// in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
}

/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to get up to date information about the chat (current name of the user for
/// one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat
/// object on success.
#[derive(Serialize, Debug, Response)]
#[response = "Chat"]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to get a list of administrators in a chat. On success, returns an Array of
/// ChatMember objects that contains information about all chat administrators except other bots.
/// If the chat is a group or a supergroup and no administrators were appointed, only the creator
/// will be returned.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<ChatMember>"]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Serialize, Debug, Response)]
#[response = "u64"]
pub struct GetChatMembersCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to get information about a member of a chat. Returns a ChatMember object on success.
#[derive(Serialize, Debug, Response)]
#[response = "ChatMember"]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator
/// in the chat for this to work and must have the appropriate admin rights. Use the field
/// can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatID,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

/// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator
/// in the chat for this to work and must have the appropriate admin rights. Use the field
/// can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatID,
}

/// Use this method to send answers to callback queries sent from inline keyboards. The answer will be
/// displayed to the user as a notification at the top of the chat screen or as an alert. On success,
/// True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200
    /// characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If true, an alert will be shown by the client instead of a notification at the top of the
    /// chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<Boolean>,
    /// URL that will be opened by the user's client. If you have created a Game and accepted the
    /// conditions via @Botfather, specify the URL that opens your game – note that this will only
    /// work if the query comes from a callback_game button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached
    /// client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<Integer>,
}

/// Use this method to change the list of the bot's commands. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetMyCommands {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands.
    /// At most 100 commands can be specified.
    pub commands: Vec<BotCommand>,
}

/// Use this method to get the current list of the bot's commands. Requires no parameters.
/// Returns Array of BotCommand on success.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<BotCommand>"]
pub struct GetMyCommands {}

/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited Message is returned, otherwise
/// True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageText {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New text of the message
    pub text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or
    /// inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<Boolean>,
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit captions of messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited Message is returned, otherwise
/// True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageCaption {
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
    /// New caption of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit audio, document, photo, or video messages. If a message is a part
/// of a message album, then it can be edited only to a photo or a video. Otherwise, message
/// type can be changed arbitrarily. When inline message is edited, new file can't be uploaded.
/// Use previously uploaded file via its file_id or specify a URL. On success, if the edited
/// message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageMedia {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    /// or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatID>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit only the reply markup of messages sent by the bot or via the bot
/// (for inline bots). On success, if edited message is sent by the bot, the edited Message
/// is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageReplyMarkup {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to stop a poll which was sent by the bot. On success,
/// the stopped Poll with the final results is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Poll"]
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Identifier of the original message with the poll
    pub message_id: Integer,
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to delete a message, including service messages, with the following limitations:
/// - A message can only be deleted if it was sent less than 48 hours ago.
/// - Bots can delete outgoing messages in groups and supergroups.
/// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// - If the bot is an administrator of a group, it can delete any message there.
/// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Identifier of the message to delete
    pub message_id: Integer,
}

/// Use this method to send .webp stickers. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendSticker {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL as a String for Telegram to get a .webp file
    /// from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub sticker: InputFileString,
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

/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[derive(Serialize, Debug, Response)]
#[response = "StickerSet"]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}

/// Use this method to upload a .png file with a sticker for later use in createNewStickerSet and
/// addStickerToSet methods (can be used multiple times). Returns the uploaded File on success.
#[derive(Serialize, Debug, Response)]
#[response = "File"]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: Integer,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed
    /// 512px, and either width or height must be exactly 512px. More info on Sending Files »
    pub png_sticker: InputFile,
}

/// Use this method to create new sticker set owned by a user. The bot will be able to edit the
/// created sticker set. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub user_id: Integer,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain
    /// only english letters, digits and underscores. Must begin with a letter, can't contain
    /// consecutive underscores and must end in “_by_<bot username>”. <bot_username> is case
    /// insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed
    /// 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send
    /// a file that already exists on the Telegram servers, pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using multipart/form-data.
    /// More info on Sending Files »
    pub png_sticker: InputFileString,
    /// TGS animation with the sticker, uploaded using multipart/form-data.
    /// See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// Pass True, if a set of mask stickers should be created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_masks: Option<Boolean>,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

/// Use this method to add a new sticker to a set created by the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub user_id: Integer,
    /// Sticker set name
    pub name: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed
    /// 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send
    /// a file that already exists on the Telegram servers, pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using multipart/form-data.
    /// More info on Sending Files »
    pub png_sticker: InputFileString,
    // TGS animation with the sticker, uploaded using multipart/form-data.
    /// See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

/// Use this method to move a sticker in a set created by the bot to a specific position . Returns
/// True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: Integer,
}

/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: String,
}

/// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set
/// for animated sticker sets only. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetStickerSetThumb {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: Integer,
    /// A PNG image with the thumbnail, must be up to 128 kilobytes in size and have width and height
    /// exactly 100px, or a TGS animation with the thumbnail up to 32 kilobytes in size;
    /// see https://core.telegram.org/animated_stickers#technical-requirements for animated sticker
    /// technical requirements. Pass a file_id as a String to send a file that already exists on the
    /// Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data. More info on Sending Files ». Animated sticker
    /// set thumbnail can't be uploaded via HTTP URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
}

/// Use this method to send answers to an inline query. On success, True is returned.
/// No more than 50 results per query are allowed.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on
    /// the server. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<Integer>,
    /// Pass True, if results may be cached on the server side only for the user that sent the
    /// query. By default, results may be returned to any user who sends the same query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<Boolean>,
    /// Pass the offset that a client should send in the next query with the same text to receive
    /// more results. Pass an empty string if there are no more results or if you don‘t support
    /// pagination. Offset length can’t exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches the user to a
    /// private chat with the bot and sends the bot a start message with the parameter
    /// switch_pm_parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,
    /// Deep-linking parameter for the /start message sent to the bot when user presses the switch
    /// button. 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

/// Use this method to send invoices. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendInvoice {
    /// Unique identifier for the target private chat
    pub chat_id: Integer,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payments provider token, obtained via Botfather
    pub provider_token: String,
    /// Unique deep-linking parameter that can be used to generate this invoice when used as a start parameter
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount,
    /// delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// A JSON-serialized data about the invoice, which will be shared with the payment provider.
    /// A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    /// People like it better when they see what they are paying for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<Integer>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<Integer>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<Integer>,
    /// Pass True, if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<Boolean>,
    /// Pass True, if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<Boolean>,
    /// Pass True, if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<Boolean>,
    /// Pass True, if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<Boolean>,
    /// Pass True, if user's phone number should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<Boolean>,
    /// Pass True, if user's email address should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<Boolean>,
    /// Pass True, if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<Boolean>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown.
    // If not empty, the first button must be a Pay button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified,
/// the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply
/// to shipping queries. On success, True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False if there are any problems
    /// (for example, if delivery to the specified address is not possible)
    pub ok: Boolean,
    /// Required if ok is True. A JSON-serialized array of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Required if ok is False. Error message in human readable form that explains why it is impossible
    /// to complete the order (e.g. "Sorry, delivery to your desired address is unavailable').
    /// Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Once the user has confirmed their payment and shipping details, the Bot API sends the final
/// confirmation in the form of an Update with the field pre_checkout_query. Use this method to
/// respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must
/// receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.) and the bot is ready to
    /// proceed with the order. Use False if there are any problems.
    pub ok: Boolean,
    /// Required if ok is False. Error message in human readable form that explains the reason for
    /// failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our
    /// amazing black T-shirts while you were busy filling out your payment details. Please choose
    /// a different color or garment!"). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Informs a user that some of the Telegram Passport elements they provided contains errors.
/// The user will not be able to re-submit their Passport to you until the errors are fixed (the
/// contents of the field for which you returned the error must change). Returns True on success.
///
/// Use this if the data submitted by the user doesn't satisfy the standards your service requires
/// for any reason. For example, if a birthday date seems invalid, a submitted document is blurry,
/// a scan shows evidence of tampering, etc. Supply some details in the error message to make sure
/// the user knows how to correct the issues.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetPassportDataErrors {
    /// User identifier
    pub user_id: Integer,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}

/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: Integer,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games
    /// via Botfather.
    pub game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title'
    /// button will be shown. If not empty, the first button must launch the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to set the score of the specified user in a game. On success, if the message
/// was sent by the bot, returns the edited Message, otherwise returns True. Returns an error,
/// if the new score is not greater than the user's current score in the chat and force is False.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct SetGameScore {
    /// User identifier
    user_id: Integer,
    /// New score, must be non-negative
    score: Integer,
    /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes
    /// or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<Boolean>,
    /// Pass True, if the game message should not be automatically edited to include the current
    /// scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<Boolean>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Integer>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// Use this method to get data for high score tables. Will return the score of the specified user
/// and several of their neighbors in a game. On success, returns an Array of GameHighScore objects.
///
/// This method will currently return scores for the target user, plus two of their closest
/// neighbors on each side. Will also return the top three users if the user and his neighbors
/// are not among them. Please note that this behavior is subject to change.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<GameHighScore>"]
pub struct GetGameHighScores {
    /// Target user id
    user_id: Integer,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Integer>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
