use types::*;
use serde_json::to_string;

#[derive(Serialize, Debug)]
pub struct SendLocationParams {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatID,
    /// Latitude of the location
    pub latitude: Float,
    /// Longitude of the location
    pub longitude: Float,
    /// Period in seconds for which the location will be updated (see Live Locations, should be
    /// between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom
    /// reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendLocationParams {
    pub fn new(chat_id: ChatID, latitude: Float, longitude: Float) -> Self {
        SendLocationParams {
            chat_id,
            latitude,
            longitude,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn live_period(&mut self, v: Integer) -> &mut Self {
        self.live_period = Some(v);
        self
    }

    pub fn disable_notification(&mut self, v: Boolean) -> &mut Self {
        self.disable_notification = Some(v);
        self
    }

    pub fn reply_to_message_id(&mut self, v: Integer) -> &mut Self {
        self.reply_to_message_id = Some(v);
        self
    }

    pub fn reply_markup(&mut self, v: ReplyMarkup) -> &mut Self {
        self.reply_markup = Some(v);
        self
    }

    pub fn json(&self) -> String {
        to_string(self).unwrap()
    }
}
