use netc::client::Client;
use rutel_derive::Response;
use serde::Serialize;
use serde_json::{from_slice, from_value, Value};

use crate::{
    bot_command::{BotCommand, BotCommandScope},
    chat_boost::UserChatBoosts,
    error::{Error, Result},
    files::File,
    games::GameHighScore,
    inline_mode::{InlineQueryResult, InlineQueryResultsButton, SentWebAppMessage},
    input_media::InputMedia,
    message::{Message, MessageEntity, MessageId, ReplyParameters},
    passport::PassportElementError,
    payments::{LabeledPrice, ShippingOption, StarTransactions},
    poll::{InputPollOption, Poll},
    reactions::ReactionType,
    stickers::{InputSticker, MaskPosition, Sticker, StickerSet},
    types::{
        Boolean, BotDescription, BotName, BotShortDescription, ChatFullInfo, ChatAdministratorRights,
        ChatID, ChatInviteLink, ChatMember, ChatPermissions, Float, ForumTopic,
        InlineKeyboardMarkup, InputFile, InputFileString, Integer, LinkPreviewOptions, MenuButton,
        ReplyMarkup, Response, TrueMessage, Update, User, UserProfilePhotos,
    },
};

#[derive(Debug)]
pub struct Bot {
    pub token: String,
    pub proxy: Option<String>,
    // pub user: Option<User>,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        Bot {
            token: token.to_string(),
            proxy: None,
            // user: None,
        }
    }

    pub fn proxy(&mut self, proxy: String) {
        self.proxy = Some(proxy);
    }

    pub fn build_uri(&self, method: &'static str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }

    pub async fn create_request(&mut self, method: &'static str, values: String) -> Result<Value> {
        let uri = self.build_uri(method);

        let client_builder = if let Some(proxy) = &self.proxy {
            Client::builder().proxy(proxy)
        } else {
            Client::builder()
        };

        let mut client = client_builder
            .post(&uri)
            .body(values)
            .header("Content-Type", "application/json")
            .build()
            .await?;
        let response = client.send().await?;
        let body = response.text()?;
        let response = body.as_bytes().to_vec();

        let v: Value = from_slice(&response)?;
        let r: Response = from_value(v)?;
        if r.ok {
            let res: Value = r.result.ok_or(Error::NoResult)?;
            Ok(res)
        } else {
            let description = r.description.ok_or(Error::NoDescription)?;
            Err(Error::Description(description))
        }
    }
}

/// A simple method for testing your bot's auth token. Requires no parameters. Returns basic information about the bot in form of a User object.
#[derive(Serialize, Debug, Response)]
#[response = "User"]
pub struct GetMe {}

/// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct LogOut {}

/// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct Close {}

// // setwebhook
// // deleteWebhook
// // getWebhookInfo

/// Use this method to receive incoming updates using long polling (wiki). An Array of Update objects is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<Update>"]
pub struct GetUpdates {
    /// Optional. Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Integer>,
    /// Optional. Limits the number of updates to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>,
    /// Optional. Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Integer>,
    /// Optional. List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to send text messages. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendMessage {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Text of the message to be sent
    pub text: String,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in message text, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to forward messages of any kind. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: ChatID,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: Integer,
}

/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of MessageId of the sent messages is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<MessageId>"]
pub struct ForwardMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Unique identifier for the chat where the original messages were sent (or channel username in the format @channelusername)
    pub from_chat_id: ChatID,
    /// Identifiers of 1-100 messages in the chat from_chat_id to forward. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<Integer>,
    /// Optional. Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the forwarded messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
}

/// Use this method to copy messages of any kind. The method is analogous to the method forwardMessages, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success.
#[derive(Serialize, Debug, Response)]
#[response = "MessageId"]
pub struct CopyMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: ChatID,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: Integer,
    /// Optional. New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the new caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the new caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessages, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of MessageId of the sent messages is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<MessageId>"]
pub struct CopyMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Unique identifier for the chat where the original messages were sent (or channel username in the format @channelusername)
    pub from_chat_id: ChatID,
    /// Identifiers of 1-100 messages in the chat from_chat_id to copy. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<Integer>,
    /// Optional. Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Pass True to copy the messages without their captions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<Boolean>,
}

/// Use this method to send photos. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendPhoto {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. More info on Sending Files »
    pub photo: InputFileString,
    /// Optional. Photo caption (may also be used when resending photos by file_id), 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    // List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Pass True if the photo needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<Boolean>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
/// For sending voice messages, use the sendVoice method instead.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendAudio {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub audio: InputFileString,
    /// Optional. Audio caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    // List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Optional. Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Optional. Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileString>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendDocument {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub document: InputFileString,
    /// Optional. Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileString>,
    /// Optional. Document caption (may also be used when resending documents by file_id), 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Disables automatic server-side content type detection for files uploaded using multipart/form-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<Boolean>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send video files, Telegram clients support mp4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendVideo {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. More info on Sending Files »
    pub video: InputFileString,
    /// Optional. Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Optional. Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileString>,
    /// Optional. Video caption (may also be used when resending videos by file_id), 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Pass True if the video needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<Boolean>,
    /// Optional. Pass True, if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<Boolean>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendAnimation {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Animation to send. Pass a file_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. More info on Sending Files »
    pub animation: InputFileString,
    /// Optional. Duration of sent animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Optional. Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Optional. Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Optional. Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileString>,
    /// Optional. Animation caption (may also be used when resending animation by file_id), 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    // List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Pass True if the animation needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<Boolean>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .ogg file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendVoice {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub voice: InputFileString,
    /// Optional. Voice message caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    // List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendVideoNote {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. More info on Sending Files ». Sending video notes by a URL is currently unsupported
    pub video_note: InputFileString,
    /// Optional. Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Optional. Video width and height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Integer>,
    /// Optional. Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileString>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send a group of photos or videos as an album. On success, an array of the sent Messages is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<Message>"]
pub struct SendMediaGroup {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    pub media: Vec<InputMedia>,
    /// Optional. Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}

/// Use this method to send point on the map. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendLocation {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Latitude of the location
    pub latitude: Float,
    /// Longitude of the location
    pub longitude: Float,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<Float>,
    /// Optional. Period in seconds for which the location will be updated (see Live Locations, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    /// Optional. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<Integer>,
    /// Optional. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<Integer>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to edit live location messages sent by the bot or via the bot (for inline bots). A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageLiveLocation {
    /// Optional. Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: Float,
    /// Longitude of new location
    pub longitude: Float,
    /// Optional	New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current live_period by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then live_period remains unchanged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<Float>,
    /// Optional. Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<Integer>,
    /// Optional. Maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<Integer>,
    /// Optional. A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to stop updating a live location message sent by the bot or via the bot (for inline bots) before live_period expires. On success, if the message was sent by the bot, the sent Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct StopMessageLiveLocation {
    /// Optional. Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Optional. A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendVenue {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Latitude of the venue
    pub latitude: Float,
    /// Longitude of the venue
    pub longitude: Float,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send phone contacts. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendContact {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send a native poll. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendPoll {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Poll question, 1-255 characters
    pub question: String,
    /// Optional. Mode for parsing entities in the question. See formatting options for more details. Currently, only custom emoji entities are allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<String>,
    /// A JSON-serialized list of 2-10 answer options
    pub options: Vec<InputPollOption>,
    /// Optional. True, if the poll needs to be anonymous, defaults to True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<Boolean>,
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Optional. True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<Boolean>,
    /// Optional. 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<Integer>,
    /// Optional. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Optional. Mode for parsing entities in the explanation. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    // List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Optional. Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<Integer>,
    /// Optional. Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<Integer>,
    /// Optional. Pass True, if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<Boolean>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendDice {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Optional. Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
///    Example: The ImageBot needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use sendChatAction with action = upload_photo. The user will see a “sending photo” status for the bot. We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SendChatAction {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_voice or upload_voice for voice notes, upload_document for general files, choose_sticker for stickers, find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: String,
}

/// Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetMessageReaction {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    pub message_id: Integer,
    /// Optional. New list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Vec<ReactionType>>,
    /// Optional. Pass True to set the reaction with a big animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<Boolean>,
}

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Serialize, Debug, Response)]
#[response = "UserProfilePhotos"]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Optional. Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Integer>,
    /// Optional. Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>,
}

/// Use this method to get basic info about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link [https://api.telegram.org/file/bot<token>/<file_path>](https://api.telegram.org/file/bot<token>/<file_path>), where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
#[derive(Serialize, Debug, Response)]
#[response = "File"]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct BanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Optional. Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Integer>,
    /// Optional. Pass True to delete all messages from the chat for the user that is being removed. If False, the user will be able to see messages in the group that were sent before the user was removed. Always True for supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<Boolean>,
}

/// Use this method to unban a previously kicked user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter only_if_banned. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @username)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Optional. Do nothing if the user is not banned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<Boolean>,
}

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate admin rights. Pass True for all boolean parameters to lift restrictions from a user. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    /// Optional. Pass True if chat permissions are set independently. Otherwise, the can_send_other_messages and can_add_web_page_previews permissions will imply the can_send_messages, can_send_audios, can_send_documents, can_send_photos, can_send_videos, can_send_video_notes, and can_send_voice_notes permissions; the can_send_polls permission will imply the can_send_messages permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<Boolean>,
    /// Optional. Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Integer>,
}

/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Pass False for all boolean parameters to demote a user. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Optional. Pass True, if the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<Boolean>,
    /// Optional. Pass True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<Boolean>,
    /// Optional. Pass True, if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<Boolean>,
    /// Optional. Pass True, if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<Boolean>,
    /// Optional. Pass True, if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<Boolean>,
    /// Optional. Pass True, if the administrator can manage video chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<Boolean>,
    /// Optional. Pass True, if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<Boolean>,
    /// Optional. Pass True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<Boolean>,
    /// Optional. Pass True, if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<Boolean>,
    /// Optional. Pass True, if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<Boolean>,
    /// Optional. Pass True, if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
    /// Optional. True, if the administrator can post stories in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<Boolean>,
    /// Optional. True, if the administrator can edit stories posted by other users; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<Boolean>,
    /// Optional. True, if the administrator can delete stories posted by other users; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<Boolean>,
    /// Optional. Pass True if the user is allowed to create, rename, close, and reopen forum topics, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<Boolean>,
}

/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
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

/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct BanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: Integer,
}

/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnbanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: Integer,
}

/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatPermissions {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// New default chat permissions
    pub permissions: ChatPermissions,
    /// Optional. Pass True if chat permissions are set independently. Otherwise, the can_send_other_messages and can_add_web_page_previews permissions will imply the can_send_messages, can_send_audios, can_send_documents, can_send_photos, can_send_videos, can_send_video_notes, and can_send_voice_notes permissions; the can_send_polls permission will imply the can_send_messages permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<Boolean>,
}

/// Use this method to generate a new invite link for a chat; any previously generated link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the new invite link as String on success.
#[derive(Serialize, Debug, Response)]
#[response = "String"]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object.
#[derive(Serialize, Debug, Response)]
#[response = "ChatInviteLink"]
pub struct CreateChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<Integer>,
    /// Optional. Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<Integer>,
    /// Optional. True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<Boolean>,
}

/// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the edited invite link as a ChatInviteLink object.
#[derive(Serialize, Debug, Response)]
#[response = "ChatInviteLink"]
pub struct EditChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// The invite link to edit
    pub invite_link: String,
    /// Optional. Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<Integer>,
    /// Optional. Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<Integer>,
    /// Optional. True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<Boolean>,
}

/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the revoked invite link as ChatInviteLink object.
#[derive(Serialize, Debug, Response)]
#[response = "ChatInviteLink"]
pub struct RevokeChatInviteLink {
    /// Unique identifier of the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// The invite link to revoke
    pub invite_link: String,
}

/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeclineChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}

/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatTitle {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// New chat title, 1-255 characters
    pub title: String,
}

/// Use this method to change the description of a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Use this method to pin a message in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct PinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Identifier of a message to pin
    pub message_id: Integer,
    /// Optional. Pass True, if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
}

/// Use this method to unpin a message in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Identifier of a message to unpin. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
}

/// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' admin right in a supergroup or 'can_edit_messages' admin right in a channel. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnpinAllChatMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to get up-to-date information about the chat. Returns a ChatFullInfo object on success.
#[derive(Serialize, Debug, Response)]
#[response = "ChatFullInfo"]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to get a list of administrators in a chat. On success, returns an Array of ChatMember objects that contains information about all chat administrators except other bots. If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<ChatMember>"]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Serialize, Debug, Response)]
#[response = "Integer"]
pub struct GetChatMemberCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatID,
}

/// Use this method to get information about a member of a chat. Returns a ChatMember object on success.
#[derive(Serialize, Debug, Response)]
#[response = "ChatMember"]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

/// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters.
///  Returns an Array of Sticker objects.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<Sticker>"]
pub struct GetForumTopicIconStickers {}

/// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights.
///  Returns information about the created topic as a ForumTopic object.
#[derive(Serialize, Debug, Response)]
#[response = "ForumTopic"]
pub struct CreateForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Topic name, 1-128 characters
    pub name: String,
    /// Optional. Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<Integer>,
    /// Optional. Unique identifier of the custom emoji shown as the topic icon. Use getForumTopicIconStickers to get all allowed custom emoji identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights, unless it is the creator of the topic.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct EditForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: Integer,
    /// Optional. New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. New unique identifier of the custom emoji shown as the topic icon. Use getForumTopicIconStickers to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct CloseForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: Integer,
}

/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct ReopenForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: Integer,
}

/// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_delete_messages administrator rights.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: Integer,
}

/// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnpinAllForumTopicMessages {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: Integer,
}

/// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct EditGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// New topic name, 1-128 characters
    pub name: String,
}

/// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct CloseGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically unhidden if it was hidden.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct ReopenGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically closed if it was open.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct HideGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnhideGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup.
/// Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct UnpinAllGeneralForumTopicMessages {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Optional. Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional. If true, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<Boolean>,
    /// Optional. URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @Botfather, specify the URL that opens your game – note that this will only work if the query comes from a callback_game button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<Integer>,
}

/// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a UserChatBoosts object.
#[derive(Serialize, Debug, Response)]
#[response = "UserChatBoosts"]
pub struct GetUserChatBoosts {
    /// Unique identifier for the chat or username of the channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

/// Use this method to change the list of the bot's commands. See [commands](https://core.telegram.org/bots#commands) for more details about bot commands. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetMyCommands {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    pub commands: Vec<BotCommand>,
    /// Optional. A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    /// Optional. A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteMyCommands {
    /// Optional. A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    /// Optional. A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to get the current list of the bot's commands for the given scope and user language. Returns Array of BotCommand on success. If commands aren't set, an empty list is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<BotCommand>"]
pub struct GetMyCommands {
    /// Optional. A JSON-serialized object, describing scope of users. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    /// Optional. A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to change the bot's name. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetMyName {
    /// Optional. New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to get the current bot name for the given user language. Returns BotName on success.
#[derive(Serialize, Debug, Response)]
#[response = "BotName"]
pub struct GetMyName {
    /// Optional. A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<BotCommand>"]
pub struct SetMyDescription {
    /// Optional. New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to get the current bot description for the given user language. Returns BotDescription on success.
#[derive(Serialize, Debug, Response)]
#[response = "BotDescription"]
pub struct GetMyDescription {
    /// Optional. A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetMyShortDescription {
    /// Optional. New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    /// Optional. A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success.
#[derive(Serialize, Debug, Response)]
#[response = "BotShortDescription"]
pub struct GetMyShortDescription {
    /// Optional. A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetChatMenuButton {
    /// Optional. Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Integer>,
    /// Optional. A JSON-serialized object for the bot's new menu button. Defaults to MenuButtonDefault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success.
#[derive(Serialize, Debug, Response)]
#[response = "MenuButton"]
pub struct GetChatMenuButton {
    /// Optional. Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Integer>,
}

// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are are free to modify the list before adding the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetMyDefaultAdministratorRights {
    /// Optional. A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    /// Optional. Pass True to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<Boolean>,
}

/// Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success.
#[derive(Serialize, Debug, Response)]
#[response = "ChatAdministratorRights"]
pub struct GetMyDefaultAdministratorRights {
    /// Optional. Pass True to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<Boolean>,
}

/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots). On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageText {
    /// Optional. Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New text of the message
    pub text: String,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    // List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Optional. A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit captions of messages sent by the bot or via the bot (for inline bots). On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageCaption {
    /// Optional. Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Optional. Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatID>,
    /// Optional. Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Optional. New caption of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    // List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit audio, document, photo, or video messages. If a message is a part of a message album, then it can be edited only to a photo or a video. Otherwise, message type can be changed arbitrarily. When inline message is edited, new file can't be uploaded. Use previously uploaded file via its file_id or specify a URL. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageMedia {
    /// Optional. Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Optional. Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatID>,
    /// Optional. Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia,
    /// Optional. A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit only the reply markup of messages sent by the bot or via the bot (for inline bots). On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct EditMessageReplyMarkup {
    /// Optional. Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Optional. A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll with the final results is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Poll"]
pub struct StopPoll {
    /// Optional. Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Identifier of the original message with the poll
    pub message_id: Integer,
    /// Optional. A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to delete a message, including service messages, with the following limitations:
/// - A message can only be deleted if it was sent less than 48 hours ago.
/// - Service messages about a supergroup, channel, or forum topic creation can't be deleted.
/// - A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.
/// - Bots can delete outgoing messages in private chats, groups, and supergroups.
/// - Bots can delete incoming messages in private chats.
/// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// - If the bot is an administrator of a group, it can delete any message there.
/// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
///  Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Identifier of the message to delete
    pub message_id: Integer,
}

/// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Identifiers of 1-100 messages to delete. See deleteMessage for limitations on which messages can be deleted
    pub message_ids: Vec<Integer>,
}

/// Use this method to send .webp stickers. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendSticker {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatID,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .webp file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub sticker: InputFileString,
    /// Optional. Emoji associated with the sticker; only for just uploaded stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
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

/// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<Sticker>"]
pub struct GetCustomEmojiStickers {
    /// List of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    pub custom_emoji_ids: Vec<String>,
}

/// Use this method to upload a file with a sticker for later use in the createNewStickerSet and addStickerToSet methods (the file can be used multiple times). Returns the uploaded File on success.
#[derive(Serialize, Debug, Response)]
#[response = "File"]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: Integer,
    /// A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See https://core.telegram.org/stickers for technical requirements. More information on Sending Files »
    pub sticker: InputFile,
    /// Format of the sticker, must be one of “static”, “animated”, “video”
    pub sticker_format: String,
}

/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub user_id: Integer,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in "_by_<bot_username>". <bot_username> is case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    pub stickers: Vec<InputSticker>,
    /// Optional. Type of stickers in the set, pass “regular”, “mask”, or “custom_emoji”. By default, a regular sticker set is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    /// Optional. Pass True if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<Boolean>,
}

/// Use this method to add a new sticker to a set created by the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub user_id: Integer,
    /// Sticker set name
    pub name: String,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    pub sticker: InputSticker,
}

/// Use this method to move a sticker in a set created by the bot to a specific position . Returns True on success.
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

/// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling deleteStickerFromSet, then addStickerToSet, then setStickerPositionInSet. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct ReplaceStickerInSet {
    /// User identifier of the sticker set owner
    pub user_id: Integer,
    /// Sticker set name
    pub name: String,
    /// File identifier of the replaced sticker
    pub old_sticker: String,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
    pub sticker: InputSticker,
}

/// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetStickerEmojiList {
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized list of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
}

/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetStickerKeywords {
    /// File identifier of the sticker
    pub sticker: String,
    /// Optional. A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

/// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetStickerMaskPosition {
    /// File identifier of the sticker
    pub sticker: String,
    /// Optional. A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

/// Use this method to set the title of a created sticker set. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetStickerSetTitle {
    /// Sticker set name
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
}

/// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set for animated sticker sets only. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetStickerSetThumbnail {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: Integer,
    /// Optional. A PNG image with the thumbnail, must be up to 128 kilobytes in size and have width and height exactly 100px, or a TGS animation with the thumbnail up to 32 kilobytes in size; see [technical-requirements](https://core.telegram.org/animated_stickers#technical-requirements) for animated sticker technical requirements. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files ». Animated sticker set thumbnail can't be uploaded via HTTP URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileString>,
    /// Yes	Format of the thumbnail, must be one of “static” for a .WEBP or .PNG image, “animated” for a .TGS animation, or “video” for a WEBM video
    pub format: String,
}

/// Use this method to set the thumbnail of a custom emoji sticker set. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetCustomEmojiStickerSetThumbnail {
    /// Sticker set name
    pub name: String,
    /// Optional. Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

/// Use this method to delete a sticker set that was created by the bot. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct DeleteStickerSet {
    /// Sticker set name
    pub name: String,
}

/// Use this method to send answers to an inline query. On success, True is returned. No more than 50 results per query are allowed.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,
    /// Optional. The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<Integer>,
    /// Optional. Pass True, if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<Boolean>,
    /// Optional. Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don‘t support pagination. Offset length can’t exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// Optional. A JSON-serialized object describing a button to be shown above inline query results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<InlineQueryResultsButton>,
}

/// Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned.
#[derive(Serialize, Debug, Response)]
#[response = "SentWebAppMessage"]
pub struct AnswerWebAppQuery {
    /// Unique identifier for the query to be answered
    pub web_app_query_id: String,
    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
}

/// Use this method to send invoices. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendInvoice {
    /// Unique identifier for the target private chat
    pub chat_id: Integer,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Optional. Payment provider token, obtained via @BotFather. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token:	Option<String>,	
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// Optional. The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<Integer>,
    /// Optional. A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<Integer>>,
    /// Optional. Unique deep-linking parameter. If left empty, forwarded copies of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    /// Optional. A JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// Optional. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Optional. Photo size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<Integer>,
    /// Optional. Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<Integer>,
    /// Optional. Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<Integer>,
    /// Optional. Pass True, if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<Boolean>,
    /// Optional. Pass True, if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<Boolean>,
    /// Optional. Pass True, if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<Boolean>,
    /// Optional. Pass True, if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<Boolean>,
    /// Optional. Pass True, if user's phone number should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<Boolean>,
    /// Optional. Pass True, if user's email address should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<Boolean>,
    /// Optional. Pass True, if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<Boolean>,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
#[derive(Serialize, Debug, Response)]
#[response = "String"]
pub struct CreateInvoiceLink {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Optional. Payment provider token, obtained via @BotFather. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token:	Option<String>,	
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// Optional. The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<Integer>,
    /// Optional. A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Integer>,
    /// Optional. JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// Optional. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Optional. Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<Integer>,
    /// Optional. Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<Integer>,
    /// Optional. Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<Integer>,
    /// Optional. Pass True if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<Boolean>,
    /// Optional. Pass True if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<Boolean>,
    /// Optional. Pass True if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<Boolean>,
    /// Optional. Pass True if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<Boolean>,
    /// Optional. Pass True if the user's phone number should be sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<Boolean>,
    /// Optional. Pass True if the user's email address should be sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<Boolean>,
    /// Optional. Pass True if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<Boolean>,
}

/// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: Boolean,
    /// Optional. Required if ok is True. A JSON-serialized array of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Optional. Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use False if there are any problems.
    pub ok: Boolean,
    /// Optional. Required if ok is False. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Returns the bot's Telegram Star transactions in chronological order. On success, returns a StarTransactions object.
#[derive(Serialize, Debug, Response)]
#[response = "StarTransactions"]
pub struct GetStarTransactions {
    /// Optional. Number of transactions to skip in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset:	Option<Integer>,	
    /// Optional. The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit:	Option<Integer>,	
}

/// Refunds a successful payment in Telegram Stars. Returns True on success.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct RefundStarPayment {
    /// Yes	Identifier of the user whose payment will be refunded
    pub user_id:	Integer	,
    /// Yes	Telegram payment identifier
    pub telegram_payment_charge_id:	String,	
}

// ------------------ PASSPORT -----------------

/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success. Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
#[derive(Serialize, Debug, Response)]
#[response = "Boolean"]
pub struct SetPassportDataErrors {
    /// User identifier
    pub user_id: Integer,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}

// ------------------ GAMES -------------------

/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Serialize, Debug, Response)]
#[response = "Message"]
pub struct SendGame {
    /// Optional Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat
    pub chat_id: Integer,
    /// Optional. Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
    pub game_short_name: String,
    /// Optional. Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// Optional. Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<Boolean>,
    /// Optional. Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Optional. Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Optional. A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to set the score of the specified user in a game. On success, if the message was sent by the bot, returns the edited Message, otherwise returns True. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
#[derive(Serialize, Debug, Response)]
#[response = "TrueMessage"]
pub struct SetGameScore {
    /// User identifier
    user_id: Integer,
    /// New score, must be non-negative
    score: Integer,
    /// Optional. Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<Boolean>,
    /// Optional. Pass True, if the game message should not be automatically edited to include the current scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<Boolean>,
    /// Optional. Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Integer>,
    /// Optional. Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. On success, returns an Array of GameHighScore objects. This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and his neighbors are not among them. Please note that this behavior is subject to change.
#[derive(Serialize, Debug, Response)]
#[response = "Vec<GameHighScore>"]
pub struct GetGameHighScores {
    /// Target user id
    user_id: Integer,
    /// Optional. Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Integer>,
    /// Optional. Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
