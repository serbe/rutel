use tokio_core::reactor::Core;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use serde_json::{from_slice, from_value, Value};
use hyper::header::{ContentLength, ContentType};
use types::{Message, Response, Update, User};
use std::io;
use futures::future::Future;
use futures::Stream;
use types::*;

use params;

#[derive(Debug)]
pub struct Bot {
    token: String,
    event_loop: Core,
    client: Client<HttpsConnector<HttpConnector>, Body>,
    user: Option<User>
}

impl Bot {
    pub fn new(token: &str) -> Self {
        let core = Core::new().unwrap();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap())
            .build(&core.handle());
        Bot {
            token: token.to_string(),
            event_loop: core,
            client,
            user: None,
        }
    }

    fn build_uri(&self, method: &'static str) -> Uri {
        let uri: Uri = format!("https://api.telegram.org/bot{}/{}", self.token, method)
            .parse()
            .unwrap();
        uri
    }

    fn create_request(&mut self, method: &'static str, values: String) -> Result<Value, String> {
        let uri = self.build_uri(method);
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(values.len() as u64));
        req.set_body(values);
        let work = self.client.request(req).and_then(|res| {
            res.body().concat2().and_then(move |body| {
                let v: Value = from_slice(&body)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                Ok(v)
            })
        });
        let v: Value = self.event_loop.run(work).map_err(|e| e.to_string())?;
        let r: Response = from_value(v).map_err(|e| e.to_string())?;
        if r.ok {
            let res: Value = r.result.ok_or("result is none")?;
            Ok(res)
        } else {
            let par = r.parameters.ok_or("Error but parameters is none")?;
            Err(par.to_string())
        }
    }

    /// A simple method for testing your bot's auth token. Requires no parameters. Returns basic
    /// information about the bot in form of a User object.
    pub fn get_me(&mut self) -> Result<User, String> {
        let resp = self.create_request("getMe", String::new())?;
        let user: User = from_value(resp).map_err(|e| e.to_string()).unwrap();
        self.user = Some(user.clone());
        Ok(user)
    }

    /// Use this method to receive incoming updates using long polling (wiki). An Array of Update
    /// objects is returned.
    ///
    /// Please note that this parameter doesn't affect updates created before the call to the
    /// get_updates, so unwanted updates may be received for a short period of time.
    /// Notes
    /// 1. This method will not work if an outgoing webhook is set up.
    /// 2. In order to avoid getting duplicate updates, recalculate offset after each server
    /// response.
    pub fn get_updates(&mut self, v: &mut params::GetUpdatesParams) -> Result<Vec<Update>, String> {
        let resp = self.create_request("getUpdates", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send text messages. On success, the sent Message is returned.
    ///
    /// Formatting options
    /// The Bot API supports basic formatting for messages. You can use bold and italic text, as
    /// well as inline links and pre-formatted code in your bots' messages. Telegram clients will
    /// render them accordingly. You can use either markdown-style or HTML-style formatting.
    ///
    /// Note that Telegram clients will display an alert to the user before opening an inline link
    /// (‘Open this link?’ together with the full URL).
    ///
    /// Links 'tg://user?id=<user_id>' can be used to mention a user by their id without using a
    /// username. Please note:
    ///
    /// These links will work only if they are used inside an inline link.
    /// These mentions are only guaranteed to work if the user has contacted the bot in the past or
    /// is a member in the group where he was mentioned.
    ///
    /// Markdown style
    /// To use this mode, pass Markdown in the parse_mode field when using send_message. Use the
    /// following syntax in your message:
    /// *bold text*
    /// _italic text_
    /// [inline URL](http://www.example.com/)
    /// [inline mention of a user](tg://user?id=123456789)
    /// `inline fixed-width code`
    /// ```block_language
    /// pre-formatted fixed-width code block
    /// ```
    ///
    /// HTML style
    /// To use this mode, pass HTML in the parse_mode field when using send_message. The following
    /// tags are currently supported:
    ///
    /// <b>bold</b>, <strong>bold</strong>
    /// <i>italic</i>, <em>italic</em>
    /// <a href="http://www.example.com/">inline URL</a>
    /// <a href="tg://user?id=123456789">inline mention of a user</a>
    /// <code>inline fixed-width code</code>
    /// <pre>pre-formatted fixed-width code block</pre>
    ///
    /// Please note:
    ///
    /// Only the tags mentioned above are currently supported.
    /// Tags must not be nested.
    /// All <, > and & symbols that are not a part of a tag or an HTML entity must be replaced with
    /// the corresponding HTML entities (< with &lt;, > with &gt; and & with &amp;).
    /// All numerical HTML entities are supported.
    /// The API currently supports only the following named HTML entities: &lt;, &gt;, &amp; and
    /// &quot;.
    pub fn send_message(&mut self, v: &mut params::SendMessageParams) -> Result<Message, String> {
        let resp = self.create_request("sendMessage", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to forward messages of any kind. On success, the sent Message is returned.
    pub fn forward_message(&mut self, v: &mut params::ForwardMessageParams) -> Result<Message, String> {
        let resp = self.create_request("forwardMessage", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    ///Use this method to send photos. On success, the sent Message is returned.
    pub fn send_photo(&mut self, v: &mut params::SendPhotoParams) -> Result<Message, String> {
        let resp = self.create_request("sendPhoto", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send audio files, if you want Telegram clients to display them in the
    /// music player. Your audio must be in the .mp3 format. On success, the sent Message is
    /// returned. Bots can currently send audio files of up to 50 MB in size, this limit may be
    /// changed in the future.
    ///
    /// For sending voice messages, use the sendVoice method instead.
    pub fn send_audio(&mut self, v: &mut params::SendAudioParams) -> Result<Message, String> {
        let resp = self.create_request("sendAudio", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send general files. On success, the sent Message is returned. Bots can
    /// currently send files of any type of up to 50 MB in size, this limit may be changed in the
    /// future.
    pub fn send_document(&mut self, v: &mut params::SendDocumentParams) -> Result<Message, String> {
        let resp = self.create_request("sendDocument", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send video files, Telegram clients support mp4 videos (other formats may
    /// be sent as Document). On success, the sent Message is returned. Bots can currently send
    /// video files of up to 50 MB in size, this limit may be changed in the future.
    pub fn send_video(&mut self, v: &mut params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request("sendVideo", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send audio files, if you want Telegram clients to display the file as a
    /// playable voice message. For this to work, your audio must be in an .ogg file encoded with
    /// OPUS (other formats may be sent as Audio or Document). On success, the sent Message is
    /// returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be
    /// changed in the future.
    pub fn send_voice(&mut self, v: &mut params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request("sendVoice", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long. Use
    /// this method to send video messages. On success, the sent Message is returned.
    pub fn send_video_note(&mut self, v: &mut params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request("sendVideoNote", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send a group of photos or videos as an album. On success, an array of
    /// the sent Messages is returned.
    pub fn send_media_group(&mut self, v: &mut params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request("sendMediaGroup", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send point on the map. On success, the sent Message is returned.
    pub fn send_location(&mut self, v: &mut params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request("sendLocation", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to edit live location messages sent by the bot or via the bot (for inline
    /// bots). A location can be edited until its live_period expires or editing is explicitly
    /// disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent
    /// by the bot, the edited Message is returned, otherwise True is returned.
    pub fn edit_message_live_location(&mut self, v: &mut params::SendVideoParams) -> Result<TrueMessage, String> {
        let resp = self.create_request("editMessageLiveLocation", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to stop updating a live location message sent by the bot or via the bot
    /// (for inline bots) before live_period expires. On success, if the message was sent by the
    /// bot, the sent Message is returned, otherwise True is returned.
    pub fn stop_message_live_location(&mut self, v: &mut params::StopMessageLiveLocationParams) -> Result<TrueMessage, String> {
        let resp = self.create_request("stopMessageLiveLocation", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send information about a venue. On success, the sent Message is returned.
    pub fn send_venue(&mut self, v: &mut params::SendVenueParams) -> Result<Message, String> {
        let resp = self.create_request("sendVenue", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send phone contacts. On success, the sent Message is returned.
    pub fn send_contact(&mut self, v: &mut params::SendContactParams) -> Result<Message, String> {
        let resp = self.create_request("sendContact", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method when you need to tell the user that something is happening on the bot's
    /// side. The status is set for 5 seconds or less (when a message arrives from your bot,
    /// Telegram clients clear its typing status). Returns True on success.
    ///
    /// Example: The ImageBot needs some time to process a request and upload the image. Instead of
    /// sending a text message along the lines of “Retrieving image, please wait…”, the bot may use
    /// sendChatAction with action = upload_photo. The user will see a “sending photo” status for
    /// the bot.
    ///
    /// We only recommend using this method when a response from the bot will take a noticeable
    /// amount of time to arrive.
    pub fn send_chat_action(&mut self, v: &mut params::SendChatAction) -> Result<Boolean, String> {
        let resp = self.create_request("sendChatAction", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos
    /// object.
    pub fn get_user_profile_photos(&mut self, v: &mut params::GetUserProfilePhotosParams) -> Result<Message, String> {
        let resp = self.create_request("getUserProfilePhotos", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to get basic info about a file and prepare it for downloading. For the
    /// moment, bots can download files of up to 20MB in size. On success, a File object is
    /// returned. The file can then be downloaded via the link
    /// https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the
    /// response. It is guaranteed that the link will be valid for at least 1 hour. When the link
    /// expires, a new one can be requested by calling getFile again.
    ///
    /// Note: This function may not preserve the original file name and MIME type. You should save
    /// the file's MIME type and name (if available) when the File object is received.
    pub fn get_file(&mut self, v: &mut params::GetFileParams) -> Result<File, String> {
        let resp = self.create_request("getFile", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to kick a user from a group, a supergroup or a channel. In the case of
    /// supergroups and channels, the user will not be able to return to the group on their own
    /// using invite links, etc., unless unbanned first. The bot must be an administrator in the
    /// chat for this to work and must have the appropriate admin rights. Returns True on success.
    ///
    /// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members
    /// Are Admins’ setting is off in the target group. Otherwise members may only be removed by
    /// the group's creator or by the member that added them.
    pub fn kick_chat_member(&mut self, v: &mut params::KickChatMemberParams) -> Result<Boolean, String> {
        let resp = self.create_request("kickChatMember", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to unban a previously kicked user in a supergroup or channel. The user will
    /// not return to the group or channel automatically, but will be able to join via link, etc.
    /// The bot must be an administrator for this to work. Returns True on success.
    pub fn unban_chat_member(&mut self, v: &mut params::UnbanChatMemberParams) -> Result<Boolean, String> {
        let resp = self.create_request("unbanChatMember", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the
    /// supergroup for this to work and must have the appropriate admin rights. Pass True for all
    /// boolean parameters to lift restrictions from a user. Returns True on success.
    pub fn restrict_chat_member(&mut self, v: &mut params::RestrictChatMemberParams) -> Result<Boolean, String> {
        let resp = self.create_request("restrictChatMember", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be
    /// an administrator in the chat for this to work and must have the appropriate admin rights.
    /// Pass False for all boolean parameters to demote a user. Returns True on success.
    pub fn promote_chat_member(&mut self, v: &mut params::PromoteChatMemberParams) -> Result<Boolean, String> {
        let resp = self.create_request("promoteChatMember", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to export an invite link to a supergroup or a channel. The bot must be an
    /// administrator in the chat for this to work and must have the appropriate admin rights.
    /// Returns exported invite link as String on success.
    pub fn export_chat_invite_link(&mut self, v: &mut params::ExportChatInviteLinkParams) -> Result<String, String> {
        let resp = self.create_request("exportChatInviteLink", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to set a new profile photo for the chat. Photos can't be changed for
    /// private chats. The bot must be an administrator in the chat for this to work and must have
    /// the appropriate admin rights. Returns True on success.
    ///
    /// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members
    /// Are Admins’ setting is off in the target group.
    pub fn set_chat_photo(&mut self, v: &mut params::SetChatPhotoParams) -> Result<Boolean, String> {
        let resp = self.create_request("setChatPhoto", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot
    /// must be an administrator in the chat for this to work and must have the appropriate admin
    /// rights. Returns True on success.
    ///
    /// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members
    /// Are Admins’ setting is off in the target group.
    pub fn delete_chat_photo(&mut self, v: &mut params::DeleteChatPhotoParams) -> Result<Boolean, String> {
        let resp = self.create_request("deleteChatPhoto", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to change the title of a chat. Titles can't be changed for private chats.
    /// The bot must be an administrator in the chat for this to work and must have the appropriate
    /// admin rights. Returns True on success.
    ///
    /// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members
    /// Are Admins’ setting is off in the target group.
    pub fn set_chat_title(&mut self, v: &mut params::SetChatTitleParams) -> Result<Boolean, String> {
        let resp = self.create_request("setChatTitle", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to change the description of a supergroup or a channel. The bot must be an
    /// administrator in the chat for this to work and must have the appropriate admin rights.
    /// Returns True on success.
    pub fn set_chat_description(&mut self, v: &mut params::SetChatDescriptionParams) -> Result<Boolean, String> {
        let resp = self.create_request("setChatDescription", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to pin a message in a supergroup or a channel. The bot must be an
    /// administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right
    /// in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on
    /// success.
    pub fn pin_chat_message(&mut self, v: &mut params::PinChatMessageParams) -> Result<Boolean, String> {
        let resp = self.create_request("pinChatMessage", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to unpin a message in a supergroup or a channel. The bot must be an
    /// administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right
    /// in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on
    /// success.
    pub fn unpin_chat_message(&mut self, v: &mut params::UnpinChatMessageParams) -> Result<Boolean, String> {
        let resp = self.create_request("unpinChatMessage", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method for your bot to leave a group, supergroup or channel. Returns True on
    /// success.
    pub fn leave_chat(&mut self, v: &mut params::LeaveChatParams) -> Result<Boolean, String> {
        let resp = self.create_request("leaveChat", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to get up to date information about the chat (current name of the user for
    /// one-on-one conversations, current username of a user, group or channel, etc.). Returns a
    /// Chat object on success.
    pub fn get_chat(&mut self, v: &mut params::GetChatParams) -> Result<Chat, String> {
        let resp = self.create_request("getChat", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to get a list of administrators in a chat. On success, returns an Array of
    /// ChatMember objects that contains information about all chat administrators except other
    /// bots. If the chat is a group or a supergroup and no administrators were appointed, only the
    /// creator will be returned.
    pub fn get_chat_administrators(&mut self, v: &mut params::GetChatAdministratorsParams) -> Result<Vec<ChatMember>, String> {
        let resp = self.create_request("getChatAdministrators", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to get the number of members in a chat. Returns Int on success.
    pub fn get_chat_members_count(&mut self, v: &mut params::GetChatMembersCountParams) -> Result<Integer, String> {
        let resp = self.create_request("getChatMembersCount", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to get information about a member of a chat. Returns a ChatMember object on
    /// success.
    pub fn get_chat_member(&mut self, v: &mut params::GetChatMemberParams) -> Result<ChatMember, String> {
        let resp = self.create_request("getChatMember", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to set a new group sticker set for a supergroup. The bot must be an
    /// administrator in the chat for this to work and must have the appropriate admin rights. Use
    /// the field can_set_sticker_set optionally returned in getChat requests to check if the bot
    /// can use this method. Returns True on success.
    pub fn set_chat_sticker_set(&mut self, v: &mut params::SetChatStickerSetParams) -> Result<Boolean, String> {
        let resp = self.create_request("setChatStickerSet", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to delete a group sticker set from a supergroup. The bot must be an
    /// administrator in the chat for this to work and must have the appropriate admin rights. Use
    /// the field can_set_sticker_set optionally returned in getChat requests to check if the bot
    /// can use this method. Returns True on success.
    pub fn delete_chat_sticker_set(&mut self, v: &mut params::DeleteChatPhotoParams) -> Result<Boolean, String> {
        let resp = self.create_request("deleteChatStickerSet", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send answers to callback queries sent from inline keyboards. The answer
    /// will be displayed to the user as a notification at the top of the chat screen or as an
    /// alert. On success, True is returned.
    ///
    /// Alternatively, the user can be redirected to the specified Game URL. For this option to
    /// work, you must first create a game for your bot via @Botfather and accept the terms.
    /// Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a
    /// parameter.
    ///
    /// Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a
    /// parameter.
    pub fn answer_callback_query(&mut self, v: &mut params::AnswerCallbackQueryParams) -> Result<Message, String> {
        let resp = self.create_request("answerCallbackQuery", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to edit text and game messages sent by the bot or via the bot (for inline
    /// bots). On success, if edited message is sent by the bot, the edited Message is returned,
    /// otherwise True is returned.
    pub fn edit_message_text(&mut self, v: &mut params::EditMessageTextParams) -> Result<TrueMessage, String> {
        let resp = self.create_request("editMessageText", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to edit captions of messages sent by the bot or via the bot (for inline
    /// bots). On success, if edited message is sent by the bot, the edited Message is returned,
    /// otherwise True is returned.
    pub fn edit_message_caption(&mut self, v: &mut params::EditMessageCaptionParams) -> Result<Message, String> {
        let resp = self.create_request("editMessageCaption", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to edit only the reply markup of messages sent by the bot or via the bot
    /// (for inline bots). On success, if edited message is sent by the bot, the edited Message is
    /// returned, otherwise True is returned.
    pub fn edit_message_reply_markup(&mut self, v: &mut params::EditMessageReplyMarkupParams) -> Result<Message, String> {
        let resp = self.create_request("editMessageReplyMarkup", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to delete a message, including service messages, with the following
    /// limitations:
    /// - A message can only be deleted if it was sent less than 48 hours ago.
    /// - Bots can delete outgoing messages in groups and supergroups.
    /// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
    /// - If the bot is an administrator of a group, it can delete any message there.
    /// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
    /// Returns True on success.
    pub fn delete_message(&mut self, v: &mut params::DeleteMessageParams) -> Result<Boolean, String> {
        let resp = self.create_request("deleteMessage", v.to_string())?;
        from_value(resp).map_err(|e| e.to_string())
    }
}
