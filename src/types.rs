use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Integer = i64;
pub type Float = f64;
pub type Boolean = bool;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ChatID {
    String(String),
    Integer(Integer),
}

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

impl From<i32> for ChatID {
    fn from(id: i32) -> Self {
        ChatID::Integer(id as i64)
    }
}

/// A JSON-serialized object
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup,
    PubReplyKeyboardMarkup,
    ReplyKeyboardRemove,
    ForceReply,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InputFileString {
    String(String),
    File(File),
}

impl From<String> for InputFileString {
    fn from(url: String) -> Self {
        InputFileString::String(url)
    }
}

impl From<File> for InputFileString {
    fn from(file: File) -> Self {
        InputFileString::File(file)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TrueMessage {
    Message(Box<Message>),
    Boolean(Boolean),
}

impl From<Message> for TrueMessage {
    fn from(message: Message) -> Self {
        TrueMessage::Message(Box::new(message))
    }
}

impl From<Boolean> for TrueMessage {
    fn from(bool: Boolean) -> Self {
        TrueMessage::Boolean(bool)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputFile {
    pub name: String,
    pub source: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Response {
    pub ok: Boolean,
    pub description: Option<String>,
    pub result: Option<Value>,
    pub error_code: Option<i64>,
    pub parameters: Option<Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    pub poll: Option<Poll>,
    pub poll_answer: Option<PollAnswer>,
}

/// Contains information about the current status of a webhook.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: Boolean,
    pub pending_update_count: Integer,
    pub last_error_date: Option<Integer>,
    pub last_error_message: Option<String>,
    pub max_connections: Option<Integer>,
    pub allowed_updates: Option<Vec<String>>,
}

/// This object represents a Telegram user or bot.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Integer,
    pub is_bot: Boolean,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub can_join_groups: Option<Boolean>,
    pub can_read_all_group_messages: Option<Boolean>,
    pub supports_inline_queries: Option<Boolean>,
}

/// This object represents a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: Integer,
    #[serde(rename = "type")]
    pub kind: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<Integer>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<Boolean>,
}

/// This object represents a message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: Integer,
    /// Sender, empty for messages sent to channels
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. The channel itself for
    /// channel messages. The supergroup itself for messages from anonymous group
    /// administrators. The linked channel for messages automatically forwarded to
    /// the discussion group
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Conversation the message belongs to
    pub chat: Box<Chat>,
    /// For forwarded messages, sender of the original message
    pub forward_from: Option<User>,
    /// For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    pub forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<Integer>,
    /// For messages forwarded from channels, signature of the post author if present
    pub forward_signature: Option<String>,
    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    pub forward_sender_name: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<Integer>,
    /// For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<Integer>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    pub animation: Option<Animation>,
    /// Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file
    pub document: Option<Document>,
    /// Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// Message is a video, information about the video
    pub video: Option<Video>,
    /// Message is a video note, information about the video message
    pub video_note: Option<VideoNote>,
    /// Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// Caption for the animation, audio, document, photo, video or voice, 0-1024 characters
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// Message is a dice with random value from 1 to 6
    pub dice: Option<Dice>,
    /// Message is a game, information about the game.
    pub game: Option<Game>,
    /// Message is a native poll, information about the poll
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    pub venue: Option<Venue>,
    /// Message is a shared location, information about the location
    pub location: Option<Location>,
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot itself)
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted
    /// True
    pub delete_chat_photo: Option<Boolean>,
    /// Service message: the group has been created
    /// True
    pub group_chat_created: Option<Boolean>,
    /// Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    /// True
    pub supergroup_chat_created: Option<Boolean>,
    /// Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    /// True
    pub channel_chat_created: Option<Boolean>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<Integer>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_from_chat_id: Option<Integer>,
    /// Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    /// Message is an invoice for a payment, information about the invoice.
    pub invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment.
    pub successful_payment: Option<SuccessfulPayment>,
    /// The domain name of the website on which the user has logged in.
    pub connected_website: Option<String>,
    /// Telegram Passport data
    pub passport_data: Option<PassportData>,
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// This object represents one special entity in a text message. For example, hashtags, usernames,
/// URLs, etc.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub kind: String,
    pub offset: Integer,
    pub length: Integer,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: Integer,
    pub height: Integer,
    pub file_size: Option<Integer>,
}

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: Integer,
    pub height: Integer,
    pub duration: Integer,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: Integer,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
    pub thumb: Option<PhotoSize>,
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

/// This object represents a video file.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: Integer,
    pub height: Integer,
    pub duration: Integer,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: Integer,
    pub duration: Integer,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<Integer>,
}

/// This object represents a voice note.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: Integer,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

/// This object represents a phone contact.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<Integer>,
    pub vcard: Option<String>,
}

/// This object represents an animated emoji that displays a random value.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for “🎲” and “🎯” base emoji, 1-5 for “🏀” and “⚽” base emoji, 1-64 for “🎰” base emoji
    pub value: Integer,
}

/// This object contains information about one answer option in a poll.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PollOption {
    pub text: String,
    pub voter_count: Integer,
}

/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<Integer>,
}

/// This object contains information about a poll.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: Integer,
    pub is_closed: Boolean,
    pub is_anonymous: Boolean,
    #[serde(rename = "type")]
    pub kind: String,
    pub allows_multiple_answers: Boolean,
    pub correct_option_id: Option<Integer>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<Integer>,
    pub close_date: Option<Integer>,
}

/// This object represents a point on the map.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Location {
    pub longitude: Float,
    pub latitude: Float,
    pub horizontal_accuracy: Option<Float>,
    pub live_period: Option<Integer>,
    pub heading: Option<Integer>,
    pub proximity_alert_radius: Option<Integer>,
}

/// This object represents a venue.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
}

/// This object represents the content of a service message, sent whenever a user in the chat
/// triggers a proximity alert set by another user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: Integer,
}

/// This object represent a user's profile pictures.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserProfilePhotos {
    pub total_count: Integer,
    pub photos: Vec<Vec<PhotoSize>>,
}

/// This object represents a file ready to be downloaded. The file can be downloaded via the link
/// https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be
/// valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
/// Maximum file size to download is 20 MB
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<Integer>,
    pub file_path: Option<String>,
}

/// This object represents a custom keyboard with reply options (see Introduction to bots
/// fordetails and examples).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: Option<Boolean>,
    pub one_time_keyboard: Option<Boolean>,
    pub selective: Option<Boolean>,
}

/// This object represents one button of the reply keyboard. For simple text buttons String can be
/// used instead of this object to specify text of the button. Optional fields are mutually
/// exclusive.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: Option<Boolean>,
    pub request_location: Option<Boolean>,
    pub request_poll: Option<KeyboardButtonPollType>,
}

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

/// Upon receiving a message with this object, Telegram clients will remove the current custom
/// keyboard and display the default letter-keyboard. By default, custom keyboards are displayed
/// until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are
/// hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: Boolean,
    pub selective: Option<Boolean>,
}

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// This object represents one button of an inline keyboard. You must use exactly one of the
/// optional fields.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub login_url: Option<LoginUrl>,
    pub callback_data: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<Boolean>,
}

/// This object represents a parameter of the inline keyboard button used to automatically authorize a user.
/// Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram. All
/// the user needs to do is tap/click a button and confirm that they want to log in: https://core.telegram.org/file/811140015/1734/8VZFkwWXalM.97872/6127fa62d8a0bf2b3c
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<Boolean>,
}

/// This object represents an incoming callback query from a callback button in an inline keyboard.
/// If the button that originated the query was attached to a message sent by the bot, the field
/// message will be present. If the button was attached to a message sent via the bot (in inline
/// mode), the field inline_message_id will be present. Exactly one of the fields data or
/// game_short_name will be present.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface to
/// the user (act as if the user has selected the bot‘s message and tapped ’Reply'). This can be
/// extremely useful if you want to create user-friendly step-by-step interfaces without having to
/// sacrifice privacy mode.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForceReply {
    pub force_reply: Boolean,
    pub selective: Option<Boolean>,
}

/// This object represents a chat photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

/// This object contains information about one member of a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMember {
    pub user: User,
    pub status: String,
    pub custom_title: Option<String>,
    /// Owner and administrators only. True, if the user's presence in the chat is hidden
    pub is_anonymous: Option<Boolean>,
    pub until_date: Option<Integer>,
    pub can_be_edited: Option<Boolean>,
    pub can_post_messages: Option<Boolean>,
    pub can_edit_messages: Option<Boolean>,
    pub can_delete_messages: Option<Boolean>,
    pub can_restrict_members: Option<Boolean>,
    pub can_promote_members: Option<Boolean>,
    pub can_change_info: Option<Boolean>,
    pub can_invite_users: Option<Boolean>,
    pub can_pin_messages: Option<Boolean>,
    pub is_member: Option<Boolean>,
    pub can_send_messages: Option<Boolean>,
    pub can_send_media_messages: Option<Boolean>,
    pub can_send_polls: Option<Boolean>,
    pub can_send_other_messages: Option<Boolean>,
    pub can_add_web_page_previews: Option<Boolean>,
}

/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatPermissions {
    pub can_send_messages: Option<Boolean>,
    pub can_send_media_messages: Option<Boolean>,
    pub can_send_polls: Option<Boolean>,
    pub can_send_other_messages: Option<Boolean>,
    pub can_add_web_page_previews: Option<Boolean>,
    pub can_change_info: Option<Boolean>,
    pub can_invite_users: Option<Boolean>,
    pub can_pin_messages: Option<Boolean>,
}

/// This object represents a bot command.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

/// Contains information about why a request was unsuccessful.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<Integer>,
    pub retry_after: Option<Integer>,
}

/// This object represents the content of a media message to be sent. It should be one of
/// InputMediaPhoto InputMediaVideo
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum InputMedia {
    InputMediaAnimation,
    InputMediaDocument,
    InputMediaAudio,
    InputMediaPhoto,
    InputMediaVideo,
}

/// Represents a photo to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaPhoto {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
}

/// Represents a video to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaVideo {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    pub thumb: Option<InputFileString>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<Integer>,
    pub height: Option<Integer>,
    pub duration: Option<Integer>,
    pub supports_streaming: Option<Boolean>,
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaAnimation {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    pub thumb: Option<InputFileString>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<Integer>,
    pub height: Option<Integer>,
    pub duration: Option<Integer>,
}

/// Represents an audio file to be treated as music to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaAudio {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    pub thumb: Option<InputFileString>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<Integer>,
    pub performer: Option<String>,
    pub title: Option<String>,
}

/// Represents a general file to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    pub thumb: Option<InputFileString>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<Boolean>,
}

/// This object represents the contents of a file to be uploaded. Must be posted using
/// multipart/form-data in the usual way that files are uploaded via the browser.
///#[derive(Clone, Serialize, Deserialize, Debug)]
///pub struct InputFile {()}

/// ---------------------------------
/// Stickers
/// ---------------------------------

/// This object represents a sticker.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: Integer,
    pub height: Integer,
    pub is_animated: Boolean,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<Integer>,
}

/// This object represents a sticker set.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub is_animated: Boolean,
    pub contains_masks: Boolean,
    pub stickers: Vec<Sticker>,
    pub thumb: Option<PhotoSize>,
}

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: Float,
    pub y_shift: Float,
    pub scale: Float,
}

/// ---------------------------------
/// Inline mode
/// ---------------------------------

/// This object represents an incoming inline query. When the user sends an empty query, your bot
/// could return some default or trending results.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

/// This object represents one result of an inline query. Telegram clients currently support
/// results of the following 20 types:
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum InlineQueryResult {
    InlineQueryResultCachedAudio,
    InlineQueryResultCachedDocument,
    InlineQueryResultCachedGif,
    InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultCachedPhoto,
    InlineQueryResultCachedSticker,
    InlineQueryResultCachedVideo,
    InlineQueryResultCachedVoice,
    InlineQueryResultArticle,
    InlineQueryResultAudio,
    InlineQueryResultContact,
    InlineQueryResultGame,
    InlineQueryResultDocument,
    InlineQueryResultGif,
    InlineQueryResultLocation,
    InlineQueryResultMpeg4Gif,
    InlineQueryResultPhoto,
    InlineQueryResultVenue,
    InlineQueryResultVideo,
    InlineQueryResultVoice,
}

/// Represents a link to an article or web page.
#[derive(Clone, Serialize, Deserialize, Debug)]
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

/// Represents a link to a photo. By default, this photo will be sent by the user with optional
/// caption. Alternatively, you can use input_message_content to send a message with the specified
/// content instead of the photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by
/// the user with optional caption. Alternatively, you can use input_message_content to send a
/// message with the specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default,
/// this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you
/// can use input_message_content to send a message with the specified content instead of the
/// animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a page containing an embedded video player or a video file. By default,
/// this video file will be sent by the user with an optional caption. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of the video.
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must
/// replace its content using input_message_content.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVideo {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub video_width: Option<Integer>,
    pub video_height: Option<Integer>,
    pub video_duration: Option<Integer>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an mp3 audio file. By default, this audio file will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the audio.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultAudio {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub performer: Option<String>,
    pub audio_duration: Option<Integer>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a voice recording in an .ogg container encoded with OPUS. By default, this
/// voice recording will be sent by the user. Alternatively, you can use input_message_content to
/// send a message with the specified content instead of the the voice message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVoice {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub voice_duration: Option<Integer>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a file. By default, this file will be sent by the user with an optional
/// caption. Alternatively, you can use input_message_content to send a message with the specified
/// content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<Integer>,
    pub thumb_height: Option<Integer>,
}

/// Represents a location on a map. By default, the location will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the location.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultLocation {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub latitude: Float,
    pub longitude: Float,
    pub title: String,
    pub horizontal_accuracy: Option<Float>,
    pub live_period: Option<Integer>,
    pub heading: Option<Integer>,
    pub proximity_alert_radius: Option<Integer>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<Integer>,
    pub thumb_height: Option<Integer>,
}

/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of the venue.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be venue
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: Float,
    /// Longitude of the venue location in degrees
    pub longitude: Float,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

/// Represents a contact with a phone number. By default, this contact will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the contact.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultContact {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<Integer>,
    pub thumb_height: Option<Integer>,
}

/// Represents a Game.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGame {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be
/// sent by the user with an optional caption. Alternatively, you can use input_message_content to
/// send a message with the specified content instead of the photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedPhoto {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub photo_file_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this
/// animated GIF file will be sent by the user with an optional caption. Alternatively, you can use
/// input_message_content to send a message with specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedGif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the
/// Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an
/// optional caption. Alternatively, you can use input_message_content to send a message with the
/// specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be
/// sent by the user. Alternatively, you can use input_message_content to send a message with the
/// specified content instead of the sticker.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedSticker {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent
/// by the user with an optional caption. Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the file.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video file stored on the Telegram servers. By default, this video file
/// will be sent by the user with an optional caption. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of the video.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedVideo {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a voice message stored on the Telegram servers. By default, this voice
/// message will be sent by the user. Alternatively, you can use input_message_content to send a
/// message with the specified content instead of the voice message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedVoice {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an mp3 audio file stored on the Telegram servers. By default, this audio
/// file will be sent by the user. Alternatively, you can use input_message_content to send a
/// message with the specified content instead of the audio.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedAudio {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub audio_file_id: String,
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

/// This object represents the content of a message to be sent as a result of an inline query.
/// Telegram clients currently support the following 4 types:
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum InputMessageContent {
    InputTextMessageContent,
    InputLocationMessageContent,
    InputVenueMessageContent,
    InputContactMessageContent,
}

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputTextMessageContent {
    pub message_text: String,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<Boolean>,
}

//Represents the content of a location message to be sent as the result of an inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputLocationMessageContent {
    pub latitude: Float,
    pub longitude: Float,
    pub horizontal_accuracy: Option<Float>,
    pub live_period: Option<Integer>,
    pub heading: Option<Integer>,
    pub proximity_alert_radius: Option<Integer>,
}

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputVenueMessageContent {
    pub latitude: Float,
    pub longitude: Float,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
}

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
}

/// Represents a result of an inline query that was chosen by the user and sent to their chat
/// partner.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

/// ---------------------
/// Payments
/// ---------------------

/// This object represents a portion of the price for goods or services.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: Integer,
}

/// This object contains basic information about an invoice.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: Integer,
}

/// This object represents a shipping address.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

/// This object represents information about an order.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

/// This object represents one shipping option.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

/// This object contains basic information about a successful payment.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: Integer,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

/// This object contains information about an incoming shipping query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

/// This object contains information about an incoming pre-checkout query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: Integer,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}

/// Contains information about Telegram Passport data shared with the bot by the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram
/// Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Integer,
    pub file_date: Integer,
}

/// Contains information about documents or other Telegram Passport elements shared with
/// the bot by the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub kind: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

/// Contains data required for decrypting and authenticating EncryptedPassportElement.
/// See the Telegram Passport Documentation for a complete description of the data decryption
/// and authentication processes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

/// This object represents an error in the Telegram Passport element which was submitted that
/// should be resolved by the user. It should be one of:
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PassportElementError {
    PassportElementErrorDataField,
    PassportElementErrorFrontSide,
    PassportElementErrorReverseSide,
    PassportElementErrorSelfie,
    PassportElementErrorFile,
    PassportElementErrorFiles,
    PassportElementErrorTranslationFile,
    PassportElementErrorTranslationFiles,
    PassportElementErrorUnspecified,
}

/// Represents an issue in one of the data fields that was provided by the user. The error is
/// considered resolved when the field's value changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorDataField {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}

/// Represents an issue with the front side of a document. The error is considered resolved when
/// the file with the front side of the document changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFrontSide {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the reverse side of a document. The error is considered resolved when
/// the file with reverse side of the document changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorReverseSide {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the selfie with a document. The error is considered resolved when the
/// file with the selfie changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorSelfie {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with a document scan. The error is considered resolved when the file with
/// the document scan changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFile {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with a list of scans. The error is considered resolved when the list of
/// files containing the scans changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFiles {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with one of the files that constitute the translation of a document.
/// The error is considered resolved when the file changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorTranslationFile {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the translated version of a document. The error is considered
/// resolved when a file with the document translation change.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorTranslationFiles {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorUnspecified {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// This object represents a game. Use BotFather to create and edit games, their short names will
/// act as unique identifiers.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Vec<Option<MessageEntity>>,
    pub animation: Option<Animation>,
}

/// A placeholder, currently holds no information. Use BotFather to set up your game.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CallbackGame;

/// This object represents one row of the high scores table for a game.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    pub position: Integer,
    pub user: User,
    pub score: Integer,
}
