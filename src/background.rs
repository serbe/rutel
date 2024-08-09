use serde::{Deserialize, Serialize};

use crate::{
    files::Document,
    types::{Boolean, Integer},
};

/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BackgroundFill {
    BackgroundFillSolid,
    BackgroundFillGradient,
    BackgroundFillFreeformGradient,
}

/// The background is filled using the selected color.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BackgroundFillSolid {
    /// Type of the background fill, always “solid”
    #[serde(rename = "type")]
    pub kind: String,
    /// The color of the background fill in the RGB24 format
    pub color: Integer,
}

/// The background is a gradient fill.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BackgroundFillGradient {
    /// Type of the background fill, always “gradient”
    #[serde(rename = "type")]
    pub kind: String,
    /// Top color of the gradient in the RGB24 format
    pub top_color: Integer,
    /// Bottom color of the gradient in the RGB24 format
    pub bottom_color: Integer,
    /// Clockwise rotation angle of the background fill in degrees; 0-359
    pub rotation_angle: Integer,
}

/// The background is a freeform gradient that rotates after every message in the chat.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BackgroundFillFreeformGradient {
    /// Type of the background fill, always “freeform_gradient”
    #[serde(rename = "type")]
    pub kind: String,
    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    pub colors: Vec<Integer>,
}

/// This object describes the type of a background. Currently, it can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BackgroundType {
    BackgroundTypeFill,
    BackgroundTypeWallpaper,
    BackgroundTypePattern,
    BackgroundTypeChatTheme,
}

/// The background is automatically filled based on the selected colors.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BackgroundTypeFill {
    /// Type of the background, always “fill”
    #[serde(rename = "type")]
    pub kind: String,
    /// The background fill
    pub fill: BackgroundFill,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: Integer,
}

/// The background is a wallpaper in the JPEG format.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BackgroundTypeWallpaper {
    /// Type of the background, always “wallpaper”
    #[serde(rename = "type")]
    pub kind: String,
    /// Document with the wallpaper
    pub document: Document,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: Integer,
    /// Optional. True, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blurred: Option<Boolean>,
    /// Optional. True, if the background moves slightly when the device is tilted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<Boolean>,
}

/// The background is a PNG or TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BackgroundTypePattern {
    /// Type of the background, always “pattern”
    #[serde(rename = "type")]
    pub kind: String,
    /// Document with the pattern
    pub document: Document,
    /// The background fill that is combined with the pattern
    pub fill: BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background; 0-100
    pub intensity: Integer,
    /// Optional. True, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inverted: Option<Boolean>,
    /// Optional. True, if the background moves slightly when the device is tilted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<Boolean>,
}

/// The background is taken directly from a built-in chat theme.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BackgroundTypeChatTheme {
    /// Type of the background, always “chat_theme”
    #[serde(rename = "type")]
    pub kind: String,
    /// Name of the chat theme, which is usually an emoji
    pub theme_name: String,
}

/// This object represents a chat background.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatBackground {
    /// Type of the background
    #[serde(rename = "type")]
    pub kind: BackgroundType,
}
