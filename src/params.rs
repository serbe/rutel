use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug, GetSet)]
pub struct GetUpdatesParams {
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

#[derive(Serialize, Debug, GetSet)]
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

#[derive(Serialize, Debug, GetSet)]
pub struct ForwardMessageParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendPhotoParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendAudioParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendDocumentParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendVideoParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendAnimationParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendVoiceParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendVideoNoteParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendMediaGroupParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendLocationParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct EditMessageLiveLocationParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct StopMessageLiveLocationParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendVenueParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendContactParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendChatActionParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive:
    /// typing for text messages, upload_photo for photos, record_video or upload_video for videos,
    /// record_audio or upload_audio for audio files, upload_document for general files,
    /// find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: String,
}

#[derive(Serialize, Debug, GetSet)]
pub struct GetUserProfilePhotosParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct GetFileParams {
    /// File identifier to get info about
    pub file_id: String,
}

#[derive(Serialize, Debug, GetSet)]
pub struct KickChatMemberParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct UnbanChatMemberParams {
    /// Unique identifier for the target group or username of the target supergroup or channel (in
    /// the format @username)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

#[derive(Serialize, Debug, GetSet)]
pub struct RestrictChatMemberParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for
    /// more than 366 days or less than 30 seconds from the current time, they are considered to be
    /// restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Integer>,
    /// Pass True, if the user can send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<Boolean>,
    /// Pass True, if the user can send audios, documents, photos, videos, video notes and voice
    /// notes, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<Boolean>,
    /// Pass True, if the user can send animations, games, stickers and use inline bots, implies
    /// can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<Boolean>,
    /// Pass True, if the user may add web page previews to their messages, implies
    /// can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<Boolean>,
}

#[derive(Serialize, Debug, GetSet)]
pub struct PromoteChatMemberParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct ExportChatInviteLinkParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
}

#[derive(Serialize, Debug, GetSet)]
pub struct SetChatPhotoParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}

#[derive(Serialize, Debug, GetSet)]
pub struct DeleteChatPhotoParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
}

#[derive(Serialize, Debug, GetSet)]
pub struct SetChatTitleParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// New chat title, 1-255 characters
    pub title: String,
}

#[derive(Serialize, Debug, GetSet)]
pub struct SetChatDescriptionParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Debug, GetSet)]
pub struct PinChatMessageParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct UnpinChatMessageParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
}

#[derive(Serialize, Debug, GetSet)]
pub struct LeaveChatParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
}

#[derive(Serialize, Debug, GetSet)]
pub struct GetChatParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
}

#[derive(Serialize, Debug, GetSet)]
pub struct GetChatAdministratorsParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
}

#[derive(Serialize, Debug, GetSet)]
pub struct GetChatMembersCountParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
}

#[derive(Serialize, Debug, GetSet)]
pub struct GetChatMemberParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

#[derive(Serialize, Debug, GetSet)]
pub struct SetChatStickerSetParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatID,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

#[derive(Serialize, Debug, GetSet)]
pub struct DeleteChatStickerSetParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatID,
}

#[derive(Serialize, Debug, GetSet)]
pub struct AnswerCallbackQueryParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct EditMessageTextParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct EditMessageCaptionParams {
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
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Serialize, Debug, GetSet)]
pub struct EditMessageMediaParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct EditMessageReplyMarkupParams {
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

#[derive(Serialize, Debug, GetSet)]
pub struct DeleteMessageParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Identifier of the message to delete
    pub message_id: Integer,
}

#[derive(Serialize, Debug, GetSet)]
pub struct SendStickerParams {
    /// Unique identifier for the target chat or username of the target channel (in the format 
    /// @channelusername)
    pub chat_id: ChatID,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram 
    /// servers (recommended), pass an HTTP URL as a String for Telegram to get a .webp file 
    /// from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub sticker: InputFileString,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification : Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply 
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize, Debug, GetSet)]
pub struct GetStickerSetParams {
    /// Name of the sticker set
    pub name: String,
}

#[derive(Serialize, Debug, GetSet)]
pub struct UploadStickerFileParams {
    /// User identifier of sticker file owner
    pub user_id: Integer,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 
    /// 512px, and either width or height must be exactly 512px. More info on Sending Files »
    pub png_sticker: InputFile,
}

#[derive(Serialize, Debug, GetSet)]
pub struct CreateNewStickerSetParams{
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
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// Pass True, if a set of mask stickers should be created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_masks: Option<Boolean>,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[derive(Serialize, Debug, GetSet)]
pub struct AddStickerToSetParams {
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
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[derive(Serialize, Debug, GetSet)]
pub struct SetStickerPositionInSetParams {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: Integer,
}

#[derive(Serialize, Debug, GetSet)]
pub struct DeleteStickerFromSetParams {
    /// File identifier of the sticker
    pub sticker: String,
}

#[derive(Serialize, Debug, GetSet)]
pub struct AnswerInlineQueryParams {
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