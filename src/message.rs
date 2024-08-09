use serde::{Deserialize, Serialize};

use crate::{
    files::{Animation, Audio, Document, PhotoSize, Video, VideoNote, Voice},
    forum_topic::{
        ForumTopicClosed, ForumTopicCreated, ForumTopicEdited, ForumTopicReopened,
        GeneralForumTopicHidden, GeneralForumTopicUnhidden,
    },
    games::Game,
    giveaway::{Giveaway, GiveawayCompleted, GiveawayCreated, GiveawayWinners},
    passport::PassportData,
    payments::{Invoice, PaidMediaInfo, RefundedPayment, SuccessfulPayment},
    poll::Poll,
    stickers::Sticker,
    types::{
        Boolean, Chat, ChatBoostAdded, ChatID, ChatShared, Contact, Dice, InlineKeyboardMarkup,
        Integer, LinkPreviewOptions, Location, MessageAutoDeleteTimerChanged,
        ProximityAlertTriggered, Story, User, UsersShared, Venue, VideoChatEnded,
        VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted, WebAppData,
        WriteAccessAllowed,
    },
};

/// This object represents a message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: Integer,
    /// Optional. Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Optional. Sender, empty for messages sent to channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,
    /// Optional. Sender of the message, sent on behalf of a chat. The channel itself for channel messages. The supergroup itself for messages from anonymous group administrators. The linked channel for messages automatically forwarded to the discussion group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,
    /// Optional. If the sender of the message boosted the chat, the number of boosts added by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<Integer>,
    /// Optional. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<User>,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Optional. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Conversation the message belongs to
    pub chat: Box<Chat>,
    /// Optional. Information about the original message for forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<MessageOrigin>,
    /// Optional. True, if the message is sent to a forum topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<Boolean>,
    /// Optional. True, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<Boolean>,
    /// Optional. For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    /// Optional. Information about the message that is being replied to, which may come from another chat or forum topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<ExternalReplyInfo>,
    /// Optional. For replies that quote part of the original message, the quoted part of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,
    /// Optional. For replies to a story, the original story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<Story>,
    /// Optional. Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,
    /// Optional. Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<Integer>,
    /// Optional. True, if the message can't be forwarded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<Boolean>,
    /// Optional. True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<Boolean>,
    /// Optional. The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    /// Optional. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    /// Optional. For text messages, the actual UTF-8 text of the message, 0-4096 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Optional. Unique identifier of the message effect added to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<String>,
    /// Optional. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    /// Optional. Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// Optional. Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    /// Optional. Message contains paid media; information about the paid media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
    /// Optional. Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Optional. Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    /// Optional. Message is a forwarded story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    /// Optional. Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// Optional. Message is a video note, information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    /// Optional. Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    /// Optional. Caption for the animation, audio, document, photo, video or voice, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. True, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<Boolean>,
    /// Optional. Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Optional. Message is a dice with random value from 1 to 6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    /// Optional. Message is a game, information about the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    /// Optional. Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Optional. Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    /// Optional. Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Optional. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    /// Optional. A member was removed from the group, information about them (this member may be the bot itself)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,
    /// Optional. A chat title was changed to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    /// Optional. A chat photo was change to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Optional. Service message: the chat photo was deleted True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<Boolean>,
    /// Optional. Service message: the group has been created True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<Boolean>,
    /// Optional. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup. True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<Boolean>,
    /// Optional. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel. True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<Boolean>,
    /// Optional. Service message: auto-delete timer settings changed in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    /// Optional. The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<Integer>,
    /// Optional. The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<Integer>,
    /// Optional. Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    /// Optional. Message is an invoice for a payment, information about the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    /// Optional. Message is a service message about a successful payment, information about the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,
    /// Optional. Message is a service message about a refunded payment, information about the payment. More about payments »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<RefundedPayment>,
    /// Optional. Service message: a users was shared with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_shared: Option<UsersShared>,
    /// Optional. Service message: a chat was shared with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<ChatShared>,
    /// Optional. The domain name of the website on which the user has logged in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    /// Optional. Service message: the user allowed the bot added to the attachment menu to write messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,
    /// Optional. Telegram Passport data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,
    /// Optional. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// Optional. Service message: user boosted the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_added: Option<ChatBoostAdded>,
    /// Optional. Service message: forum topic created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,
    /// Optional. Service message: forum topic edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<ForumTopicEdited>,
    /// Optional. Service message: forum topic closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<ForumTopicClosed>,
    /// Optional. ssage: forum topic reopened
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    /// Optional. Service message: the 'General' forum topic hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    /// Optional. Service message: the 'General' forum topic unhidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    /// Optional. Service message: a scheduled giveaway was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<GiveawayCreated>,
    /// Optional. The message is a scheduled giveaway message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,
    /// Optional. A giveaway with public winners was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,
    /// Optional. Service message: a giveaway without public winners was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<GiveawayCompleted>,
    /// Optional. Service message: video chat scheduled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    /// Optional. Service message: video chat started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,
    /// Optional. Service message: video chat ended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,
    /// Optional. Service message: new participants invited to a video chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    /// Optional. Service message: data sent by a Web App
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,
    /// Optional. Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// This object represents a unique message identifier.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: Integer,
}

/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: Integer,
    /// Always 0. The field can be used to differentiate regular and inaccessible messages.
    pub date: Integer,
}

/// This object describes a message that can be inaccessible to the bot. It can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
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
    /// Optional. For “text_link” only, url that will be opened after user taps on the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. For “text_mention” only, the mentioned user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Optional. For “pre” only, the programming language of the entity text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Optional. For “custom_emoji” only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

/// This object contains information about the quoted part of a message that is replied to by the given message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given message
    pub text: String,
    /// Optional. Special entities that appear in the quote. Currently, only bold, italic, underline, strikethrough, spoiler, and custom_emoji entities are kept in quotes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Approximate quote position in the original message in UTF-16 code units as specified by the sender
    pub position: Integer,
    /// Optional. True, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<Boolean>,
}

/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Optional. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    /// Optional. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Optional. Options used for link preview generation for the original message, if it is a text message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Optional. Message is an animation, information about the animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    /// Optional. Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// Optional. Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    /// Optional. Message contains paid media; information about the paid media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
    /// Optional. Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Optional. Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    /// Optional. Message is a forwarded story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    /// Optional. Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// Optional. Message is a video note, information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    /// Optional. Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    /// Optional. True, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<Boolean>,
    /// Optional. Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Optional. Message is a dice with random value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    /// Optional. Message is a game, information about the game. More about games »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    /// Optional. Message is a scheduled giveaway, information about the giveaway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,
    /// Optional. A giveaway with public winners was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,
    /// Optional. Message is an invoice for a payment, information about the invoice. More about payments »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    /// Optional. Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Optional. Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Optional. Message is a venue, information about the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
}

/// Describes reply parameters for the message that is being sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat, or in the chat chat_id if it is specified
    pub message_id: Integer,
    /// Optional. If the message to be replied to is from a different chat, unique identifier for the chat or username of the channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatID>,
    /// Optional. Pass True if the message should be sent even if the specified message to be replied to is not found; can be used only for replies in the same chat and forum topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<Boolean>,
    /// Optional. Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including bold, italic, underline, strikethrough, spoiler, and custom_emoji entities. The message will fail to send if the quote isn't found in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    /// Optional. Mode for parsing entities in the quote. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,
    /// Optional. A JSON-serialized list of special entities that appear in the quote. It can be specified instead of quote_parse_mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,
    /// Optional. Position of the quote in the original message in UTF-16 code units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<Integer>,
}

/// This object describes the origin of a message. It can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MessageOrigin {
    MessageOriginUser(MessageOriginUser),
    MessageOriginHiddenUser(MessageOriginHiddenUser),
    MessageOriginChat(MessageOriginChat),
    MessageOriginChannel(MessageOriginChannel),
}

/// The message was originally sent by a known user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageOriginUser {
    /// Type of the message origin, always “user”
    #[serde(rename = "type")]
    pub kind: String,
    /// Date the message was sent originally in Unix time
    pub date: Integer,
    /// User that sent the message originally
    pub sender_user: User,
}

/// The message was originally sent by an unknown user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageOriginHiddenUser {
    /// Type of the message origin, always “hidden_user”
    #[serde(rename = "type")]
    pub kind: String,
    /// Date the message was sent originally in Unix time
    pub date: Integer,
    /// Name of the user that sent the message originally
    pub sender_user_name: String,
}

/// The message was originally sent on behalf of a chat to a group chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageOriginChat {
    /// Type of the message origin, always “chat”
    #[serde(rename = "type")]
    pub kind: String,
    /// Date the message was sent originally in Unix time
    pub date: Integer,
    /// Chat that sent the message originally
    pub sender_chat: Chat,
    /// Optional. For messages originally sent by an anonymous chat administrator, original message author signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}

/// The message was originally sent to a channel chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageOriginChannel {
    /// Type of the message origin, always “channel”
    #[serde(rename = "type")]
    pub kind: String,
    /// Date the message was sent originally in Unix time
    pub date: Integer,
    /// Channel chat to which the message was originally sent
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: Integer,
    /// Optional. Signature of the original post author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}
