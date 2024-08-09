use serde::{Deserialize, Serialize};

use crate::{
    files::{File, PhotoSize},
    types::{Boolean, Float, InputFileString, Integer},
};

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
    /// Optional. Sticker thumbnail in the .WEBP or .JPG format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Optional. Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Optional. Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    /// Optional. For premium regular stickers, premium animation for the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,
    /// Optional. For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// Optional. For custom emoji stickers, unique identifier of the custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// Optional. File size in bytes
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
    /// Type of stickers in the set, currently one of “regular”, “mask”, “custom_emoji”
    pub sticker_type: String,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Optional. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
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

/// This object describes a sticker to be added to a sticker set.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputSticker {
    /// The added sticker. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, upload a new one using multipart/form-data, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. Animated and video stickers can't be uploaded via HTTP URL. More information on Sending Files »
    pub sticker: InputFileString,
    /// Format of the added sticker, must be one of “static” for a .WEBP or .PNG image, “animated” for a .TGS animation, “video” for a WEBM video
    pub format: String,
    /// List of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
    /// Optional. Position where the mask should be placed on faces. For “mask” stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// Optional. List of 0-20 search keywords for the sticker with total length of up to 64 characters. For “regular” and “custom_emoji” stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}
