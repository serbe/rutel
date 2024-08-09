use crate::{
    business::{
        BusinessConnection, BusinessIntro, BusinessLocation, BusinessMessagesDeleted,
        BusinessOpeningHours,
    },
    chat_boost::{ChatBoostRemoved, ChatBoostUpdated},
    files::{File, PhotoSize},
    inline_mode::{ChosenInlineResult, InlineQuery},
    keyboard_button::{InlineKeyboardButton, KeyboardButton},
    message::{MaybeInaccessibleMessage, Message},
    payments::{PreCheckoutQuery, ShippingQuery},
    poll::{Poll, PollAnswer},
    reactions::{MessageReactionCountUpdated, MessageReactionUpdated, ReactionType},
};

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

impl From<&String> for ChatID {
    fn from(id: &String) -> Self {
        ChatID::String(id.to_string())
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

/// This object describes the paid media to be sent. Currently, it can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum InputPaidMedia {
    InputPaidMediaPhoto,
    InputPaidMediaVideo,
}

/// The paid media to send is a photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputPaidMediaPhoto {
    /// Type of the media, must be photo
    #[serde(rename = "type")]
    pub kind:	String,	
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media:	String,	
}

/// The paid media to send is a video.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputPaidMediaVideo {
    /// Type of the media, must be video
    #[serde(rename = "type")]
    pub kind:	String,	
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media:	String,	
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFileString>,	
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    ///Optional. Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<Boolean>,	
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Response {
    pub ok: Boolean,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

/// Update This object represents an incoming update. At most one of the optional parameters can be present in any given update.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: Integer,
    /// Optional. New incoming message of any kind — text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// Optional. New version of a message that is known to the bot and was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,
    /// Optional. New incoming channel post of any kind — text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,
    /// Optional. New version of a channel post that is known to the bot and was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,
    /// Optional. The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection: Option<BusinessConnection>,
    /// Optional. New message from a connected business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_message: Option<Message>,
    /// Optional. New version of a message from a connected business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_business_message: Option<Message>,
    /// Optional. Messages were deleted from a connected business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_business_messages: Option<BusinessMessagesDeleted>,
    /// Optional. A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify "message_reaction" in the list of allowed_updates to receive these updates. The update isn't received for reactions set by bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<MessageReactionUpdated>,
    /// Optional. Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify "message_reaction_count" in the list of allowed_updates to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<MessageReactionCountUpdated>,
    /// Optional. New incoming inline query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,
    /// Optional. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// Optional. New incoming callback query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
    /// Optional. New incoming shipping query. Only for invoices with flexible price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,
    /// Optional. New incoming pre-checkout query. Contains full information about checkout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// Optional. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Optional. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,
    /// Optional. The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,
    /// Optional. A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify “chat_member” in the list of allowed_updates to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,
    /// Optional. A request to join the chat has been sent. The bot must have the can_invite_users administrator right in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,
    /// Optional. A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<ChatBoostUpdated>,
    /// Optional. A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<ChatBoostRemoved>,
}

/// Contains information about the current status of a webhook.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: Boolean,
    /// Number of updates awaiting delivery
    pub pending_update_count: Integer,
    /// Optional. Currently used webhook IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Optional. Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<Integer>,
    /// Optional. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// Optional. Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<Integer>,
    /// Optional. Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<Integer>,
    /// Optional. update types the bot is subscribed to. Defaults to all update types except chat_member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

/// This object represents a Telegram user or bot.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: Integer,
    /// True, if this user is a bot
    pub is_bot: Boolean,
    /// User's or bot's first name
    pub first_name: String,
    /// Optional. User's or bot's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. User's or bot's username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Optional. True, if this user is a Telegram Premium user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<Boolean>,
    /// Optional. True, if this user added the bot to the attachment menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<Boolean>,
    /// Optional. True, if the bot can be invited to groups. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<Boolean>,
    /// Optional. True, if privacy mode is disabled for the bot. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<Boolean>,
    /// Optional. True, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<Boolean>,
    /// Optional. True, if the bot supports inline queries. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<Boolean>,
}

/// This object represents a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: Integer,
    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub kind: String,
    /// Optional. Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Optional. Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. True, if the supergroup chat is a forum (has topics enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<Boolean>,
}

/// This object contains full information about a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatFullInfo {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: Integer,
    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub kind: String,
    /// Optional. Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Optional. Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. True, if the supergroup chat is a forum (has topics enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<Boolean>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    pub accent_color_id: Integer,
    /// The maximum number of reactions that can be set on a message in the chat
    pub max_reaction_count: Integer,
    /// Optional. Chat photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Optional. If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// Optional. For private chats, the date of birth of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Birthdate>,
    /// Optional. For private chats with business accounts, the intro of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,
    /// Optional. For private chats with business accounts, the location of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_location: Option<BusinessLocation>,
    /// Optional. For private chats with business accounts, the opening hours of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<BusinessOpeningHours>,
    /// Optional. For private chats, the personal channel of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Chat>,
    /// Optional. List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,
    /// Optional. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    /// Optional. Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<Integer>,
    /// Optional. Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,
    /// Optional. Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Optional. Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<Integer>,
    /// Optional. Bio of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Optional. True, if privacy settings of the other party in the private chat allows to use tg://user?id=<user_id> links only in chats with the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<Boolean>,
    /// Optional. True, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<Boolean>,
    /// Optional. True, if users need to join the supergroup before they can send messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<Boolean>,
    /// Optional. True, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<Boolean>,
    /// Optional. Description, for groups, supergroups and channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Primary invite link, for groups, supergroups and channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// Optional. The most recent pinned message (by sending date)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Message>,
    /// Optional. Default chat member permissions, for groups and supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    /// Optional. True, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<Boolean>,
    /// Optional. For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<Integer>,
    /// Optional. For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<Integer>,
    /// Optional. The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// Optional. True, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<Boolean>,
    /// Optional. True, if non-administrators can only get the list of bots and administrators in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<Boolean>,
    /// Optional. True, if messages from the chat can't be forwarded to other chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<Boolean>,
    /// Optional. True, if new chat members will have access to old messages; available only to chat administrators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<Boolean>,
    /// Optional. For supergroups, name of the group sticker set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// Optional. True, if the bot can change the group sticker set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<Boolean>,
    /// Optional. For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,
    /// Optional. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<Integer>,
    /// Optional. For supergroups, the location to which the supergroup is connected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

/// This object represents a story.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Story {
    /// Chat that posted the story
    pub chat: Chat,
    /// Unique identifier for the story in the chat
    pub id: Integer,
}

/// This object represents a phone contact.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

/// This object represents an animated emoji that displays a random value.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for “🎲”, “🎯” and “🎳” base emoji, 1-5 for “🏀” and “⚽” base emoji, 1-64 for “🎰” base emoji
    pub value: Integer,
}

/// This object represents a point on the map.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Location {
    pub longitude: Float,
    pub latitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Optional. Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

/// Contains data sent from a Web App to the bot.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this field.
    pub data: String,
    /// Text of the web_app keyboard button, from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    pub button_text: String,
}

/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: Integer,
}

/// This object represents a service message about a change in auto-delete timer settings.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat
    pub message_auto_delete_time: Integer,
}

/// This object represents a service message about a user boosting a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatBoostAdded {
    /// Number of boosts added by the user
    pub boost_count: Integer,
}

/// This object contains information about a user that was shared with the bot using a KeyboardButtonRequestUsers button.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SharedUser {
    /// Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    pub user_id: Integer,
    /// Optional. First name of the user, if the name was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Optional. Last name of the user, if the name was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_nam: Option<String>,
    /// Optional. Username of the user, if the username was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. Available sizes of the chat photo, if the photo was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

/// This object contains information about the users whose identifiers were shared with the bot using a KeyboardButtonRequestUsers button.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UsersShared {
    /// Identifier of the request
    pub request_id: Integer,
    /// Information about users shared with the bot.
    pub users: Vec<SharedUser>,
}

/// This object contains information about the chat whose identifier was shared with the bot using a KeyboardButtonRequestChat button.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: Integer,
    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    pub chat_id: Integer,
    /// Optional. Title of the chat, if the title was requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Username of the chat, if the username was requested by the bot and available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. Available sizes of the chat photo, if the photo was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

/// This object represents a service message about a user allowing a bot added to the attachment menu to write messages. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WriteAccessAllowed {
    /// Optional. True, if the access was granted after the user accepted an explicit request from a Web App sent by the method requestWriteAccess
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<Boolean>,
    /// Optional. Name of the Web App which was launched from a link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
    /// Optional. True, if the access was granted when the bot was added to the attachment or side menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<Boolean>,
}

/// This object represents a service message about a video chat scheduled in the chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: Integer,
}

/// This object represents a service message about a video chat started in the chat. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VideoChatStarted {}

/// This object represents a service message about a video chat ended in the chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VideoChatEnded {
    /// Video chat duration; in seconds
    pub duration: Integer,
}

/// This object represents a service message about new members invited to a video chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VideoChatParticipantsInvited {
    /// Optional. New members that were invited to the video chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

/// Describes the options used for link preview generation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LinkPreviewOptions {
    /// Optional. True, if the link preview is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<Boolean>,
    /// Optional. URL to use for the link preview. If empty, then the first URL found in the message text will be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. True, if the media in the link preview is suppposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<Boolean>,
    /// Optional. True, if the media in the link preview is suppposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<Boolean>,
    /// Optional. True, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<Boolean>,
}

/// This object represent a user's profile pictures.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserProfilePhotos {
    pub total_count: Integer,
    pub photos: Vec<Vec<PhotoSize>>,
}

/// Contains information about a [Web App](https://core.telegram.org/bots/webapps).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in [initializing-web-apps](https://core.telegram.org/bots/webapps#initializing-web-apps)
    pub url: String,
}

/// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Optional. Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to false, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<Boolean>,
    /// Optional. Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<Boolean>,
    /// Optional. Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat – the user can press a special button in the input field to see the custom keyboard again. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<Boolean>,
    /// Optional. The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Optional. Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<Boolean>,
}

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: Boolean,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<Boolean>,
}

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in: [https://core.telegram.org/file/811140015/1734/8VZFkwWXalM.97872/6127fa62d8a0bf2b3c](https://core.telegram.org/file/811140015/1734/8VZFkwWXalM.97872/6127fa62d8a0bf2b3c)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<Boolean>,
}

/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SwitchInlineQueryChosenChat {
    /// Optional. The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Optional. True, if private chats with users can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<Boolean>,
    /// Optional. True, if private chats with bots can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<Boolean>,
    /// Optional. True, if group and supergroup chats can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<Boolean>,
    /// Optional. True, if channel chats can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<Boolean>,
}

/// This object represents an incoming callback query from a callback button in an inline keyboard. If the button that originated the query was attached to a message sent by the bot, the field message will be present. If the button was attached to a message sent via the bot (in inline mode), the field inline_message_id will be present. Exactly one of the fields data or game_short_name will be present.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Optional. Message sent by the bot with the callback button that originated the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MaybeInaccessibleMessage>,
    /// Optional. Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    /// Optional. Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Optional. Short name of a Game to be returned, serves as the unique identifier for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    pub force_reply: Boolean,
    /// Optional. The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Optional. Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<Boolean>,
}

/// This object represents a chat photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}

/// Represents an invite link for a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: Boolean,
    /// True, if the link is primary
    pub is_primary: Boolean,
    /// True, if the link is revoked
    pub is_revoked: Boolean,
    /// Optional. Invite link name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. Point in time (Unix timestamp) when the link will expire or has been expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<Integer>,
    /// Optional. Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<Integer>,
    /// Optional. Number of pending join requests created using this link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<Integer>,
}

/// Represents the rights of an administrator in a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatAdministratorRights {
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: Boolean,
    /// True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: Boolean,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: Boolean,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: Boolean,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Boolean,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: Boolean,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: Boolean,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: Boolean,
    /// Optional. True, if the administrator can post in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<Boolean>,
    /// Optional. True, if the administrator can edit messages of other users and can pin messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<Boolean>,
    /// Optional. True, if the user is allowed to pin messages; groups and supergroups only
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
    /// Optional. True, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<Boolean>,
}

/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatMember {
    ChatMemberOwner,
    ChatMemberAdministrator,
    ChatMemberMember,
    ChatMemberRestricted,
    ChatMemberLeft,
    ChatMemberBanned,
}

/// Represents a chat member that owns the chat and has all administrator privileges.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMemberOwner {
    /// The member's status in the chat, always “creator”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Custom title for this user
    pub custom_title: String,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: Boolean,
}

/// Represents a chat member that has some additional privileges.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMemberAdministrator {
    /// The member's status in the chat, always “administrator”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: Boolean,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: Boolean,
    /// True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: Boolean,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: Boolean,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: Boolean,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Boolean,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: Boolean,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: Boolean,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: Boolean,
    /// Optional. True, if the administrator can post in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<Boolean>,
    /// Optional. he administrator can edit messages of other users and can pin messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<Boolean>,
    /// Optional. True, if the user is allowed to pin messages; groups and supergroups only
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
    /// Optional. True, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<Boolean>,
    /// Optional. Custom title for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

/// Represents a chat member that has no additional privileges or restrictions.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMemberMember {
    /// The member's status in the chat, always “member”
    pub status: String,
    /// Information about the user
    pub user: User,
}

/// Represents a chat member that is under certain restrictions in the chat. Supergroups only.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMemberRestricted {
    /// The member's status in the chat, always “restricted”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the user is a member of the chat at the moment of the request
    pub is_member: Boolean,
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: Boolean,
    /// True, if the user is allowed to send audios
    pub can_send_audios: Boolean,
    /// True, if the user is allowed to send documents
    pub can_send_documents: Boolean,
    /// True, if the user is allowed to send photos
    pub can_send_photos: Boolean,
    /// True, if the user is allowed to send videos
    pub can_send_videos: Boolean,
    /// True, if the user is allowed to send video notes
    pub can_send_video_notes: Boolean,
    /// True, if the user is allowed to send voice notes
    pub can_send_voice_notes: Boolean,
    /// True, if the user is allowed to send polls
    pub can_send_polls: Boolean,
    /// True, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: Boolean,
    /// True, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: Boolean,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: Boolean,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: Boolean,
    /// True, if the user is allowed to pin messages
    pub can_pin_messages: Boolean,
    /// True, if the user is allowed to create forum topics
    pub can_manage_topics: Boolean,
    /// Date when restrictions will be lifted for this user; unix time. If 0, then the user is restricted forever
    pub until_date: Integer,
}

/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMemberLeft {
    /// The member's status in the chat, always “left”
    pub status: String,
    /// Information about the user
    pub user: User,
}

/// Represents a chat member that was banned in the chat and can't return to the chat or view chat messages.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMemberBanned {
    /// The member's status in the chat, always “kicked”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time
    pub until_date: Integer,
}

/// This object represents changes in the status of a chat member.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: Integer,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Optional. Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    /// Optional. True, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<Boolean>,
    /// Optional. True, if the user joined the chat via a chat folder invite link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<Boolean>,
}

/// Represents a join request sent to a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 24 hours to send messages until the join request is processed, assuming no other administrator contacted the user.
    pub user_chat_id: Integer,
    /// Date the request was sent in Unix time
    pub date: Integer,
    /// Optional. Bio of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Optional. Chat invite link that was used by the user to send the join request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatPermissions {
    /// Optional. True, if the user is allowed to send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<Boolean>,
    /// Optional. True, if the user is allowed to send audios
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<Boolean>,
    /// Optional. True, if the user is allowed to send documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<Boolean>,
    /// Optional. True, if the user is allowed to send photos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<Boolean>,
    /// Optional. True, if the user is allowed to send videos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<Boolean>,
    /// Optional. True, if the user is allowed to send video notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<Boolean>,
    /// Optional. True, if the user is allowed to send voice notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<Boolean>,
    /// Optional. True, if the user is allowed to send polls, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<Boolean>,
    /// Optional. True, if the user is allowed to send animations, games, stickers and use inline bots, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<Boolean>,
    /// Optional. True, if the user is allowed to add web page previews to their messages, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<Boolean>,
    /// Optional. True, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<Boolean>,
    /// Optional. True, if the user is allowed to invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<Boolean>,
    /// Optional. True, if the user is allowed to pin messages. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
    /// Optional. True, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<Boolean>,
}

/// Describes the birthdate of a user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: Integer,
    /// Month of the user's birth; 1-12
    pub month: Integer,
    /// Optional. Year of the user's birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Integer>,
}

/// Represents a location to which a chat is connected.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}

/// This object represents a forum topic.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: Integer,
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: Integer,
    /// Optional. Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents the bot's name.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotName {
    /// The bot's name
    pub name: String,
}

/// This object represents the bot's description.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotDescription {
    /// The bot's description
    pub description: String,
}

/// This object represents the bot's short description.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotShortDescription {
    /// The bot's short description
    pub short_description: String,
}

/// This object describes the bot's menu button in a private chat. It should be one of
/// If a menu button other than MenuButtonDefault is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MenuButton {
    MenuButtonCommands(MenuButtonCommands),
    MenuButtonWebApp(MenuButtonWebApp),
    MenuButtonDefault(MenuButtonDefault),
}

/// Represents a menu button, which opens the bot's list of commands.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MenuButtonCommands {
    /// Type of the button, must be commands
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents a menu button, which launches a Web App.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MenuButtonWebApp {
    /// Type of the button, must be web_app
    #[serde(rename = "type")]
    pub kind: String,
    /// Text on the button
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery.
    pub web_app: WebAppInfo,
}

/// Describes that no specific value for the menu button was set.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MenuButtonDefault {
    /// Type of the button, must be default
    #[serde(rename = "type")]
    pub kind: String,
}

/// Contains information about why a request was unsuccessful.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseParameters {
    /// Optional. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<Integer>,
    /// Optional. In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<Integer>,
}
