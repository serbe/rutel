use serde_json::Value;

pub type Integer = u64;
pub type Float = f64;
pub type Boolean = bool;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ChatID {
    String(String),
    Integer(Integer),
}

// impl serde::Serialize for ChatID {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match *self {
//             ChatID::String(ref id) => serializer.serialize_str(id),
//             ChatID::Integer(id) => serializer.serialize_i64(id),
//         }
//     }
// }

impl From<String> for ChatID {
    fn from(id: String) -> Self {
        ChatID::String(id)
    }
}

impl From<Integer> for ChatID {
    fn from(id: Integer) -> Self {
        ChatID::Integer(id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub ok: Boolean,
    pub description: Option<String>,
    pub result: Option<Value>,
    pub error_code: Option<i64>,
    pub parameters: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: Integer,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
    pub shipping_query: Option<ShippingQuery>,
    pub pre_checkout_query: Option<PreCheckoutQuery>,
}

// This object represents a Telegram user or bot.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Integer,
    pub is_bot: Boolean,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}


// This object represents a chat.
#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: Integer,
    #[serde(rename = "type")]
    pub kind: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<Boolean>,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<Boolean>
}

// This object represents a message.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: Integer,
    pub from: Option<User>,
    pub date: Option<Integer>,
    pub chat: Option<Box<Chat>>,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<Integer>,
    pub forward_signature: Option<String>,
    pub forward_date: Option<Integer>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<Integer>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub video_note: Option<VideoNote>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    // True
    pub delete_chat_photo: Option<Boolean>,
    // True
    pub group_chat_created: Option<Boolean>,
    // True
    pub supergroup_chat_created: Option<Boolean>,
    // True
    pub channel_chat_created: Option<Boolean>,
    pub migrate_to_chat_id: Option<Integer>,
    pub migrate_from_chat_id: Option<Integer>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
}

// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub kind: String,
    pub offset: Integer,
    pub length: Integer,
    pub url: Option<String>,
    pub user: Option<User>,
}

// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub file_size: Option<Integer>,
}

// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub duration: Integer,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

// This object represents a video file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub duration: Integer,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

// This object represents a voice note.
#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub duration: Integer,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoNote {
    pub file_id: String,
    pub length: Integer,
    pub duration: Integer,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<Integer>,
}

// This object represents a phone contact.
#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<Integer>,
}

// This object represents a point on the map.
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub longitude: Float,
    pub latitude: Float,
}

// This object represents a venue.
#[derive(Serialize, Deserialize, Debug)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
}

// This object represent a user's profile pictures.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfilePhotos {
    pub total_count: Integer,
    pub photos: Vec<Vec<PhotoSize>>,
}

// This object represents a file ready to be downloaded. The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
// Maximum file size to download is 20 MB
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<Integer>,
    pub file_path: Option<String>,
}

// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples).
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: Option<Boolean>,
    pub one_time_keyboard: Option<Boolean>,
    pub selective: Option<Boolean>,
}

// This object represents one button of the reply keyboard. For simple text buttons String can be used instead of this object to specify text of the button. Optional fields are mutually exclusive.
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: Option<Boolean>,
    pub request_location: Option<Boolean>,
}

// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: Boolean,
    pub selective: Option<Boolean>,
}

// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<Boolean>,
}

// This object represents an incoming callback query from a callback button in an inline keyboard. If the button that originated the query was attached to a message sent by the bot, the field message will be present. If the button was attached to a message sent via the bot (in inline mode), the field inline_message_id will be present. Exactly one of the fields data or game_short_name will be present.
#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot‘s message and tapped ’Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode.
#[derive(Serialize, Deserialize, Debug)]
pub struct ForceReply {
    pub force_reply: Boolean,
    pub selective: Option<Boolean>,
}

// This object represents a chat photo.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}

// This object contains information about one member of a chat.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMember {
    pub user: User,
    pub status: String,
    pub until_date: Option<Integer>,
    pub can_be_edited: Option<Boolean>,
    pub can_change_info: Option<Boolean>,
    pub can_post_messages: Option<Boolean>,
    pub can_edit_messages: Option<Boolean>,
    pub can_delete_messages: Option<Boolean>,
    pub can_invite_users: Option<Boolean>,
    pub can_restrict_members: Option<Boolean>,
    pub can_pin_messages: Option<Boolean>,
    pub can_promote_members: Option<Boolean>,
    pub can_send_messages: Option<Boolean>,
    pub can_send_media_messages: Option<Boolean>,
    pub can_send_other_messages: Option<Boolean>,
    pub can_add_web_page_previews: Option<Boolean>,
}

// Contains information about why a request was unsuccessful.
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<Integer>,
    pub retry_after: Option<Integer>,
}

// This object represents the content of a media message to be sent. It should be one of InputMediaPhoto InputMediaVideo
#[derive(Serialize, Deserialize, Debug)]
pub enum InputMedia {
    InputMediaPhoto,
    InputMediaVideo,
}

// Represents a photo to be sent.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaPhoto {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    pub caption: Option<String>,
}

// Represents a video to be sent.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaVideo {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    pub caption: Option<String>,
    pub width: Option<Integer>,
    pub height: Option<Integer>,
    pub duration: Option<Integer>,
}

// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
//#[derive(Serialize, Deserialize, Debug)]
//pub struct InputFile {()}

// ---------------------------------
// Stickers
// ---------------------------------

// This object represents a sticker.
#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<Integer>,
}

// This object represents a sticker set.
#[derive(Serialize, Deserialize, Debug)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub contains_masks: Boolean,
    pub stickers: Vec<Sticker>,
}

// This object describes the position on faces where a mask should be placed by default.
#[derive(Serialize, Deserialize, Debug)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: Float,
    pub y_shift: Float,
    pub scale: Float,
}

// ---------------------------------
// Inline mode
// ---------------------------------

// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

// Represents a link to an article or web page.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultArticle {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub hide_url: Option<Boolean>,
    pub description: Option<String>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<Integer>,
    pub thumb_height: Option<Integer>,
}

// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultPhoto {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub photo_url: String,
    pub thumb_url: String,
    pub photo_width: Option<Integer>,
    pub photo_height: Option<Integer>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<Integer>,
    pub gif_height: Option<Integer>,
    pub gif_duration: Option<Integer>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<Integer>,
    pub mpeg4_height: Option<Integer>,
    pub mpeg4_duration: Option<Integer>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVideo {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub video_width: Option<Integer>,
    pub video_height: Option<Integer>,
    pub video_duration: Option<Integer>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to an mp3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultAudio {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub performer: Option<String>,
    pub audio_duration: Option<Integer>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a voice recording in an .ogg container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVoice {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub voice_duration: Option<Integer>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<Integer>,
    pub thumb_height: Option<Integer>,
}

// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultLocation {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub latitude: Float,
    pub longitude: Float,
    pub title: String,
    pub live_period: Option<Integer>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<Integer>,
    pub thumb_height: Option<Integer>,
}

// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the venue.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVenue {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub latitude: Float,
    pub longitude: Float,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<Integer>,
    pub thumb_height: Option<Integer>,
}

// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultContact {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<Integer>,
    pub thumb_height: Option<Integer>,
}

// Represents a Game.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGame {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedPhoto {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub photo_file_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with specified content instead of the animation.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedGif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the sticker.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedSticker {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedVideo {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the voice message.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedVoice {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// Represents a link to an mp3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedAudio {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub audio_file_id: String,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 4 types:
#[derive(Serialize, Deserialize, Debug)]
pub enum InputMessageContent {
    InputTextMessageContent,
    InputLocationMessageContent,
    InputVenueMessageContent,
    InputContactMessageContent,
}

// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<Boolean>,
}

//Represents the content of a location message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputLocationMessageContent {
    pub latitude: Float,
    pub longitude: Float,
    pub live_period: Option<Integer>,
}

// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputVenueMessageContent {
    pub latitude: Float,
    pub longitude: Float,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
}

// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

// ---------------------
// Payments
// ---------------------

// This object represents a portion of the price for goods or services.
#[derive(Serialize, Deserialize, Debug)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: Integer,
}

// This object contains basic information about an invoice.
#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: Integer,
}

// This object represents a shipping address.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

// This object represents information about an order.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

// This object represents one shipping option.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

// This object contains basic information about a successful payment.
#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: Integer,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

// This object contains information about an incoming shipping query.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

// This object contains information about an incoming pre-checkout query.
#[derive(Serialize, Deserialize, Debug)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: Integer,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}

// -----------------
// Games
// -----------------

// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Vec<Option<MessageEntity>>,
    pub animation: Option<Animation>,
}

// A placeholder, currently holds no information. Use BotFather to set up your game.
#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackGame;

// You can provide an animation for your game so that it looks stylish in chats (check out Lumberjack for an example). This object represents an animation file to be displayed in the message containing a game.
#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

// This object represents one row of the high scores table for a game.
#[derive(Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    pub position: Integer,
    pub user: User,
    pub score: Integer,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup,
    pubReplyKeyboardMarkup,
    ReplyKeyboardRemove,
    ForceReply,
}
