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
    /// New incoming message of any kind — text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// New version of a message that is known to the bot and was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,
    /// New version of a channel post that is known to the bot and was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,
    /// New incoming inline query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// New incoming callback query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
    /// New incoming shipping query. Only for invoices with flexible price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,
    /// New incoming pre-checkout query. Contains full information about checkout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,
    /// The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,
    /// A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify “chat_member” in the list of allowed_updates to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,
    /// A request to join the chat has been sent. The bot must have the can_invite_users administrator right in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,
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
    /// Currently used webhook IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<Integer>,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<Integer>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<Integer>,
    /// update types the bot is subscribed to. Defaults to all update types except chat_member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

/// This object represents a Telegram user or bot.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Integer,
    pub is_bot: Boolean,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<Boolean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<Boolean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<Boolean>,
}

/// This object represents a chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: Integer,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub kind: String,
    /// Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// True, if the supergroup chat is a forum (has topics enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<Boolean>,
    /// Chat photo. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// Custom emoji identifier of emoji status of the other party in a private chat. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Bio of the other party in a private chat. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// True, if privacy settings of the other party in the private chat allows to use tg://user?id=<user_id> links only in chats with the user. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<Boolean>,
    /// True, if the privacy settings of the other party restrict sending voice and video note messages in the private chat. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<Boolean>,
    /// True, if users need to join the supergroup before they can send messages. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<Boolean>,
    /// True, if all users directly joining the supergroup need to be approved by supergroup administrators. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<Boolean>,
    /// Description, for groups, supergroups and channel chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Primary invite link, for groups, supergroups and channel chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// The most recent pinned message (by sending date). Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions, for groups and supergroups. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    /// For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user; in seconds. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<Integer>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<Integer>,
    /// True, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<Boolean>,
    /// True, if non-administrators can only get the list of bots and administrators in the chat. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<Boolean>,
    /// True, if messages from the chat can't be forwarded to other chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<Boolean>,
    /// For supergroups, name of group sticker set. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<Boolean>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<Integer>,
    /// For supergroups, the location to which the supergroup is connected. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

/// This object represents a message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: Integer,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Sender, empty for messages sent to channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. The channel itself for channel messages. The supergroup itself for messages from anonymous group administrators. The linked channel for messages automatically forwarded to the discussion group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Conversation the message belongs to
    pub chat: Box<Chat>,
    /// For forwarded messages, sender of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<User>,
    /// For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<Integer>,
    /// For messages forwarded from channels, signature of the post author if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,
    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<Integer>,
    /// True, if the message is sent to a forum topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<Boolean>,
    /// True, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<Boolean>,
    /// For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<Integer>,
    /// True, if the message can't be forwarded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<Boolean>,
    /// The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    /// Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    /// Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    /// Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// Message is a video note, information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    /// Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    /// Caption for the animation, audio, document, photo, video or voice, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// True, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<Boolean>,
    /// Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Message is a dice with random value from 1 to 6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    /// Message is a game, information about the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    /// Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    /// Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot itself)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<Boolean>,
    /// Service message: the group has been created True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<Boolean>,
    /// Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup. True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<Boolean>,
    /// Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel. True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<Boolean>,
    /// Service message: auto-delete timer settings changed in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<Integer>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<Integer>,
    /// Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Message is an invoice for a payment, information about the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,
    /// Service message: a user was shared with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_shared: Option<UserShared>,
    /// Service message: a chat was shared with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<ChatShared>,
    /// The domain name of the website on which the user has logged in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    /// Service message: the user allowed the bot added to the attachment menu to write messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,
    /// Telegram Passport data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// Service message: forum topic created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,
    /// Service message: forum topic edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<ForumTopicEdited>,
    /// Service message: forum topic closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<ForumTopicClosed>,
    /// ssage: forum topic reopened
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    /// Service message: the 'General' forum topic hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    /// Service message: the 'General' forum topic unhidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    /// Service message: video chat scheduled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    /// Service message: video chat started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,
    /// Service message: video chat ended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,
    /// Service message: new participants invited to a video chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    /// Service message: data sent by a Web App
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,
    /// Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// This object represents a unique message identifier.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: Integer,
}

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    /// Type of the entity. Currently, can be “mention” (`@username`), “hashtag” (`#hashtag`), “cashtag” (`$USD`), “bot_command” (`/start@jobs_bot`), “url” (`https://telegram.org`), “email” (`do-not-reply@telegram.org`), “phone_number” (`+1-212-555-0123`), “bold” (**bold text**), “italic” (*italic text*), “underline” (<ins>underlined text</ins>), “strikethrough” (~~strikethrough text~~), “spoiler” (`spoiler message`), “code” (`monowidth string`), “pre” (`monowidth block`), “text_link” (`for clickable text URLs`), “text_mention” (`for users without usernames`)
    #[serde(rename = "type")]
    pub kind: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units
    pub length: Integer,
    /// For “text_link” only, url that will be opened after user taps on the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// For “pre” only, the programming language of the entity text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// For “custom_emoji” only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Photo width
    pub width: Integer,
    /// Photo height
    pub height: Integer,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: Integer,
    /// Video height as defined by sender
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub duration: Integer,
    /// Animation thumbnail as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: Integer,
    pub duration: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}

/// This object represents a voice note.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<Integer>,
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
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
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

/// This object represents a service message about a new forum topic created in the chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: Integer,
    /// Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopicClosed {}

/// This object represents a service message about an edited forum topic.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopicEdited {
    /// New name of the topic, if it was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForumTopicReopened {}

/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GeneralForumTopicHidden {}

/// This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GeneralForumTopicUnhidden {}

/// This object contains information about the user whose identifier was shared with the bot using a KeyboardButtonRequestUser button.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserShared {
    /// Identifier of the request
    pub request_id: Integer,
    /// Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    pub user_id: Integer,
}

/// This object contains information about the chat whose identifier was shared with the bot using a KeyboardButtonRequestChat button.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: Integer,
    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    pub chat_id: Integer,
}

/// This object represents a service message about a user allowing a bot added to the attachment menu to write messages. Currently holds no information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WriteAccessAllowed {}

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
    /// New members that were invited to the video chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

/// This object represent a user's profile pictures.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserProfilePhotos {
    pub total_count: Integer,
    pub photos: Vec<Vec<PhotoSize>>,
}

/// This object represents a file ready to be downloaded. The file can be downloaded via the link [https://api.telegram.org/file/bot<token>/<file_path>](https://api.telegram.org/file/bot<token>/<file_path>). It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile. Maximum file size to download is 20 MB
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
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
    /// Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to false, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<Boolean>,
    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<Boolean>,
    /// Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat – the user can press a special button in the input field to see the custom keyboard again. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<Boolean>,
    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<Boolean>,
}

/// This object represents one button of the reply keyboard. For simple text buttons String can be used instead of this object to specify text of the button. Optional fields are mutually exclusive.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,
    /// If specified, pressing the button will open a list of suitable users. Tapping on any user will send their identifier to the bot in a “user_shared” service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_user: Option<KeyboardButtonRequestUser>,
    /// If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a “chat_shared” service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,
    /// If True, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<Boolean>,
    /// If True, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<Boolean>,
    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    /// If specified, the described [webapps](https://core.telegram.org/bots/webapps) will be launched when the button is pressed. The Web App will be able to send a “web_app_data” service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

/// This object defines the criteria used to request a suitable user. The identifier of the selected user will be shared with the bot when the corresponding button is pressed.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButtonRequestUser {
    /// Signed 32-bit identifier of the request, which will be received back in the UserShared object. Must be unique within the message
    pub request_id: Integer,
    /// Pass True to request a bot, pass False to request a regular user. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<Boolean>,
    /// Pass True to request a premium user, pass False to request a non-premium user. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<Boolean>,
}

/// This object defines the criteria used to request a suitable chat. The identifier of the selected chat will be shared with the bot when the corresponding button is pressed.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in the ChatShared object. Must be unique within the message
    pub request_id: Integer,
    /// Pass True to request a channel chat, pass False to request a group or a supergroup chat.
    pub chat_is_channel: Boolean,
    /// Pass True to request a forum supergroup, pass False to request a non-forum chat. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<Boolean>,
    /// Pass True to request a supergroup or a channel with a username, pass False to request a chat without a username. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<Boolean>,
    /// Pass True to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<Boolean>,
    /// A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of bot_administrator_rights. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    /// A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of user_administrator_rights. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    /// Pass True to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<Boolean>,
}

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    pub kind: Option<String>,
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
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// HTTP or tg:// url to be opened when the button is pressed. Links tg://user?id=<user_id> can be used to mention a user by their ID without using a username, if this is allowed by their privacy settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery. Available only in private chats between a user and the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// An HTTP URL used to automatically authorize the user. Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. Can be empty, in which case just the bot's username will be inserted.
    /// Note: This offers an easy way for users to start using your bot in inline mode when they are currently in a private chat with it. Especially useful when combined with switch_pm… actions – in this case the user will be automatically returned to the chat they switched from, skipping the chat selection screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. Can be empty, in which case only the bot's username will be inserted.
    /// This offers a quick way for the user to open your bot in inline mode in the same chat – good for selecting something from multiple options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    /// Description of the game that will be launched when the user presses the button.
    /// NOTE: This type of button must always be the first button in the first row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,
    /// Specify True, to send a Pay button.
    /// NOTE: This type of button must always be the first button in the first row and can only be used in invoice messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<Boolean>,
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

/// This object represents an incoming callback query from a callback button in an inline keyboard. If the button that originated the query was attached to a message sent by the bot, the field message will be present. If the button was attached to a message sent via the bot (in inline mode), the field inline_message_id will be present. Exactly one of the fields data or game_short_name will be present.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    pub force_reply: Boolean,
    /// The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
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
    /// Invite link name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<Integer>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<Integer>,
    /// Number of pending join requests created using this link
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
    /// True, if the administrator can post in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<Boolean>,
    /// True, if the administrator can edit messages of other users and can pin messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<Boolean>,
    /// True, if the user is allowed to pin messages; groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
    /// True, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
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
    /// True, if the administrator can post in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<Boolean>,
    /// he administrator can edit messages of other users and can pin messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<Boolean>,
    /// True, if the user is allowed to pin messages; groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
    /// True, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<Boolean>,
    /// Custom title for this user
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
    /// Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
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
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<Boolean>,
    /// True, if the user is allowed to send audios
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<Boolean>,
    /// True, if the user is allowed to send documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<Boolean>,
    /// True, if the user is allowed to send photos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<Boolean>,
    /// True, if the user is allowed to send videos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<Boolean>,
    /// True, if the user is allowed to send video notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<Boolean>,
    /// True, if the user is allowed to send voice notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<Boolean>,
    /// True, if the user is allowed to send polls, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<Boolean>,
    /// True, if the user is allowed to send animations, games, stickers and use inline bots, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<Boolean>,
    /// True, if the user is allowed to add web page previews to their messages, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<Boolean>,
    /// True, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<Boolean>,
    /// True, if the user is allowed to invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<Boolean>,
    /// True, if the user is allowed to pin messages. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
    /// True, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<Boolean>,
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
    /// Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a bot command.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BotCommandScope {
    BotCommandScopeDefault,
    BotCommandScopeAllPrivateChats,
    BotCommandScopeAllGroupChats,
    BotCommandScopeAllChatAdministrators,
    BotCommandScopeChat,
    BotCommandScopeChatAdministrators,
    BotCommandScopeChatMember,
}

/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeDefault {
    /// Scope type, must be default
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents the scope of bot commands, covering all private chats.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeAllPrivateChats {
    /// Scope type, must be all_private_chats
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents the scope of bot commands, covering all group and supergroup chats.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeAllGroupChats {
    /// Scope type, must be all_group_chats
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeAllChatAdministrators {
    /// Scope type, must be all_chat_administrators
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents the scope of bot commands, covering a specific chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeChat {
    /// Scope type, must be chat
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Represents the scope of bot commands, covering all administrators of a specific group or supergroup chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeChatAdministrators {
    /// Scope type, must be chat_administrators
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
}

/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BotCommandScopeChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatID,
    /// Unique identifier of the target user
    pub user_id: Integer,
}

/// This object describes the bot's menu button in a private chat. It should be one of
/// If a menu button other than MenuButtonDefault is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MenuButton {
    MenuButtonCommands,
    MenuButtonWebApp,
    MenuButtonDefault,
}

/// Contains information about why a request was unsuccessful.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<Integer>,
    /// In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<Integer>,
}

/// This object represents the content of a media message to be sent. It should be one of InputMediaPhoto InputMediaVideo
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
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub kind: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: String,
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True if the photo needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<Boolean>,
}

/// Represents a video to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub kind: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: String,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<Boolean>,
    /// Pass True if the video needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<Boolean>,
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaAnimation {
    /// Type of the result, must be animation
    #[serde(rename = "type")]
    pub kind: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: String,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
    /// Caption of the animation to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Animation duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    /// Pass True if the animation needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<Boolean>,
}

/// Represents an audio file to be treated as music to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaAudio {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Represents a general file to be sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputMediaDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<Boolean>,
}

/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
///#[derive(Clone, Serialize, Deserialize, Debug)]
///pub struct InputFile {()}

/// This object represents a sticker.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Type of the sticker, currently one of “regular”, “mask”, “custom_emoji”. The type of the sticker is independent from its format, which is determined by the fields is_animated and is_video.
    #[serde(rename = "type")]
    pub kind: String,
    /// Sticker width
    pub width: Integer,
    /// Sticker height
    pub height: Integer,
    /// True, if the sticker is animated
    pub is_animated: Boolean,
    /// True, if the sticker is a video sticker
    pub is_video: Boolean,
    /// Sticker thumbnail in the .WEBP or .JPG format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// For custom emoji stickers, unique identifier of the custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}

/// This object represents a sticker set.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// True, if the sticker set contains animated stickers
    pub is_animated: Boolean,
    /// True, if the sticker set contains video stickers
    pub is_video: Boolean,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: Float,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: Float,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: Float,
}

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// Type of the chat, from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<Boolean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultPhoto {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub photo_url: String,
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub gif_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<Integer>,
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub mpeg4_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<Integer>,
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video. If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVideo {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an mp3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultAudio {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a voice recording in an .ogg container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVoice {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub document_url: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultLocation {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub latitude: Float,
    pub longitude: Float,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the venue.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultContact {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}

/// Represents a Game.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGame {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedPhoto {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub photo_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedGif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub gif_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub mpeg4_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the sticker.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedSticker {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub sticker_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedVideo {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the voice message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedVoice {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an mp3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedAudio {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub audio_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 4 types:
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
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<Boolean>,
}

//Represents the content of a location message to be sent as the result of an inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputLocationMessageContent {
    pub latitude: Float,
    pub longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<Integer>,
}

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputVenueMessageContent {
    pub latitude: Float,
    pub longitude: Float,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputContactMessageContent {
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
}

/// Represents the content of an invoice message to be sent as the result of an inline query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via Botfather
    pub provider_token: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<Integer>,
    /// A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<Integer>>,
    /// A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
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
}

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub query: String,
}

/// Contains information about an inline message sent by a [webapps](https://core.telegram.org/bots/webapps) on behalf of a user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}

/// Contains information about Telegram Passport data shared with the bot by the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Integer,
    pub file_date: Integer,
}

/// Contains information about documents or other Telegram Passport elements shared with the bot by the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

/// Contains data required for decrypting and authenticating EncryptedPassportElement. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
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

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorDataField {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}

/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFrontSide {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorReverseSide {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorSelfie {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFile {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFiles {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorTranslationFile {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
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

/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub text_entities: Vec<Option<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
