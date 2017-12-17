use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug, GetSet)]
pub struct EditMessageLiveLocationParams {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatID>,
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
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom
    /// reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocationParams {
    pub fn new(latitude: Float, longitude: Float) -> Self {
        EditMessageLiveLocationParams {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            latitude,
            longitude,
            reply_markup: None,
        }
    }
}
