use serde::{Deserialize, Serialize};

use crate::{
    message::MessageEntity,
    payments::LabeledPrice,
    types::{
        Boolean, Float, InlineKeyboardMarkup, Integer, LinkPreviewOptions, Location, User,
        WebAppInfo,
    },
};

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
    /// Optional. Type of the chat, from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// Optional. Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// This object represents a button to be shown above inline query results. You must use exactly one of the optional fields.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultsButton {
    /// Label text on the button
    pub text: String,
    /// Optional. Description of the Web App that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method switchInlineQuery inside the Web App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// Optional. Deep-linking parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.
    /// Example: An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a switch_inline button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
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
    /// Type of the result, must be article
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. Pass True if you don't want the URL to be shown in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<Boolean>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<Integer>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<Integer>,
}

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in JPEG format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumbnail_url: String,
    /// Optional. Width of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<Integer>,
    /// Optional. Height of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<Integer>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be gif
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// Optional. Width of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<Integer>,
    /// Optional. Height of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<Integer>,
    /// Optional. Duration of the GIF in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<Integer>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,
    /// Optional. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MPEG4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<Integer>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<Integer>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<Integer>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,
    /// Optional. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video. If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// MIME type of the content of the video URL, “text/html” or “video/mp4”
    pub mime_type: String,
    /// URL of the thumbnail (JPEG only) for the video
    pub thumbnail_url: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<Integer>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<Integer>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<Integer>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video. This field is required if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
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
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// A valid URL for the file
    pub document_url: String,
    /// MIME type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. URL of the thumbnail (JPEG only) for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<Integer>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<Integer>,
}

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultLocation {
    /// Type of the result, must be location
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: Float,
    /// Location longitude in degrees
    pub longitude: Float,
    /// Location title
    pub title: String,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<Float>,
    /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    /// Optional. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<Integer>,
    /// Optional. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<Integer>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<Integer>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<Integer>,
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
    /// Optional. Foursquare identifier of the venue if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<Integer>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<Integer>,
}

/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultContact {
    /// Type of the result, must be contact
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    // Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<Integer>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<Integer>,
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedGif {
    /// Type of the result, must be gif
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the MPEG4 file
    pub mpeg4_file_id: String,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video animation
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. True, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<Boolean>,
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
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
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
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
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
    /// Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
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
    /// Optional. Payment provider token, obtained via @BotFather. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// Optional. The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<Integer>,
    /// Optional. A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<Integer>>,
    /// Optional. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
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
    /// Optional. Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
