use serde::{Deserialize, Serialize};

use crate::{
    games::CallbackGame,
    types::{
        Boolean, ChatAdministratorRights, Integer, LoginUrl, SwitchInlineQueryChosenChat,
        WebAppInfo,
    },
};

/// This object represents one button of the reply keyboard. For simple text buttons String can be used instead of this object to specify text of the button. Optional fields are mutually exclusive.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,
    /// Optional. If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a “users_shared” service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,
    /// Optional. If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a “chat_shared” service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,
    /// Optional. If True, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<Boolean>,
    /// Optional. If True, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<Boolean>,
    /// Optional. If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    /// Optional. If specified, the described [webapps](https://core.telegram.org/bots/webapps) will be launched when the button is pressed. The Web App will be able to send a “web_app_data” service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

/// This object defines the criteria used to request a suitable user. The identifier of the selected user will be shared with the bot when the corresponding button is pressed.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request, which will be received back in the UserShared object. Must be unique within the message
    pub request_id: Integer,
    /// Optional. Pass True to request a bot, pass False to request a regular user. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<Boolean>,
    /// Optional. Pass True to request a premium user, pass False to request a non-premium user. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<Boolean>,
    /// Optional. The maximum number of users to be selected; 1-10. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<Integer>,
}

/// This object defines the criteria used to request a suitable chat. The identifier of the selected chat will be shared with the bot when the corresponding button is pressed.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in the ChatShared object. Must be unique within the message
    pub request_id: Integer,
    /// Pass True to request a channel chat, pass False to request a group or a supergroup chat.
    pub chat_is_channel: Boolean,
    /// Optional. Pass True to request a forum supergroup, pass False to request a non-forum chat. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<Boolean>,
    /// Optional. Pass True to request a supergroup or a channel with a username, pass False to request a chat without a username. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<Boolean>,
    /// Optional. Pass True to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<Boolean>,
    /// Optional. A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of bot_administrator_rights. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    /// Optional. A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of user_administrator_rights. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    /// Optional. Pass True to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<Boolean>,
}

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

/// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// Optional. HTTP or tg:// url to be opened when the button is pressed. Links tg://user?id=<user_id> can be used to mention a user by their ID without using a username, if this is allowed by their privacy settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Optional. Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery. Available only in private chats between a user and the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// Optional. An HTTP URL used to automatically authorize the user. Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. Can be empty, in which case just the bot's username will be inserted.
    /// Optional. Note: This offers an easy way for users to start using your bot in inline mode when they are currently in a private chat with it. Especially useful when combined with switch_pm… actions – in this case the user will be automatically returned to the chat they switched from, skipping the chat selection screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. Can be empty, in which case only the bot's username will be inserted.
    /// Optional. This offers a quick way for the user to open your bot in inline mode in the same chat – good for selecting something from multiple options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    /// Optional. If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    /// Description of the game that will be launched when the user presses the button.
    /// Optional. NOTE: This type of button must always be the first button in the first row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,
    /// Specify True, to send a Pay button.
    /// Optional. NOTE: This type of button must always be the first button in the first row and can only be used in invoice messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<Boolean>,
}
