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

#[derive(Serialize, Debug, GetSet)]
pub struct SendDocumentParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data. More info on Sending Files »
    pub document: FilePtr,
    /// Document caption (may also be used when resending documents by file_id), 0-200 characters
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendVideoParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet,
    /// or upload a new video using multipart/form-data. More info on Sending Files »
    pub video: FilePtr,
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

#[derive(Serialize, Debug, GetSet)]
pub struct SendVoiceParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the
    /// Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub voice: FilePtr,
    /// Voice message caption, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
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
    pub video_note: FilePtr,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Video width and height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Integer>,
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
    pub chat_id: ChatID,
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
