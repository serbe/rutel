use serde_json::Value;

pub type Integer = u64;
pub type Boolean = bool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    ok: Boolean,
    description: Option<String>,
    pub result: Value,
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
    pub pinned_message: Option<Message>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<Boolean>
}

// This object represents a message.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: Integer,
    pub from: Option<User>,
    pub date: Option<Integer>,
    pub chat: Option<Chat>,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<Integer>,
    pub forward_signature: Option<String>,
    pub forward_date: Option<Integer>,
    pub reply_to_message: Option<Message>,
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
    pub pinned_message: Option<Message>,
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
    pub photos: Vec<PhotoSize>,
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
    pub keyboard: Vec<KeyboardButton>,
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
    pub inline_keyboard: Vec<InlineKeyboardButton>,
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

//Inline mode objects
//Objects and methods used in the inline mode are described in the Inline mode section.