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
    //    user: Option<User>
}

impl Bot {
    pub fn new(token: &'static str) -> Self {
        let core = Core::new().unwrap();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap())
            .build(&core.handle());
        Bot {
            token: token.to_string(),
            event_loop: core,
            client,
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

//
//    /// Use this method to send text messages. On success, the sent Message is returned.
//    pub fn send_message(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendMessage", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to forward messages of any kind. On success, the sent Message is returned.
//    pub fn forward_message(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("forwardMessage", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send photos. On success, the sent Message is returned.
//    pub fn send_photo(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendPhoto", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send audio files, if you want Telegram clients to display them in the
//    /// music player. Your audio must be in the .mp3 format. On success, the sent Message is
//    /// returned. Bots can currently send audio files of up to 50 MB in size, this limit may be
//    /// changed in the future.
//    ///
//    /// For sending voice messages, use the sendVoice method instead.
//    pub fn send_audio(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendAudio", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send general files. On success, the sent Message is returned. Bots can
//    /// currently send files of any type of up to 50 MB in size, this limit may be changed in the
//    /// future.
//    pub fn send_document(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendDocument", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send video files, Telegram clients support mp4 videos (other formats may
//    /// be sent as Document). On success, the sent Message is returned. Bots can currently send
//    /// video files of up to 50 MB in size, this limit may be changed in the future.
//    pub fn send_video(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendVideo", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send audio files, if you want Telegram clients to display the file as a
//    /// playable voice message. For this to work, your audio must be in an .ogg file encoded with
//    /// OPUS (other formats may be sent as Audio or Document). On success, the sent Message is
//    /// returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be
//    /// changed in the future.
//    pub fn send_voice(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendVoice", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long.
//    /// Use this method to send video messages. On success, the sent Message is returned.
//    pub fn send_video_note(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendVideoNote", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send a group of photos or videos as an album. On success, an array of
//    /// the sent Messages is returned.
//    pub fn send_media_group(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendMediaGroup", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send point on the map. On success, the sent Message is returned.
//    pub fn send_location(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendLocation", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to edit live location messages sent by the bot or via the bot (for inline
//    /// bots). A location can be edited until its live_period expires or editing is explicitly
//    /// disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent
//    /// by the bot, the edited Message is returned, otherwise True is returned.
//    pub fn edit_message_live_location(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("editMessageLiveLocation", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to stop updating a live location message sent by the bot or via the bot
//    /// (for inline bots) before live_period expires. On success, if the message was sent by the
//    /// bot, the sent Message is returned, otherwise True is returned.
//    pub fn stop_message_live_location(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("stopMessageLiveLocation", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send information about a venue. On success, the sent Message is returned.
//    pub fn send_venue(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendVenue", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to send phone contacts. On success, the sent Message is returned.
//    pub fn send_contact(&mut self, v: String) -> Result<Message, String> {
//        let resp = self.create_request("sendContact", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method when you need to tell the user that something is happening on the bot's
//    /// side. The status is set for 5 seconds or less (when a message arrives from your bot,
//    /// Telegram clients clear its typing status). Returns True on success.
//    ///
//    /// Example: The ImageBot needs some time to process a request and upload the image. Instead
//    /// of sending a text message along the lines of “Retrieving image, please wait…”, the bot may
//    /// use sendChatAction with action = upload_photo. The user will see a “sending photo” status
//    /// for the bot.
//    ///
//    /// We only recommend using this method when a response from the bot will take a noticeable
//    /// amount of time to arrive.
//    pub fn send_chat_action(&mut self, v: String) -> Result<Boolean, String> {
//        let resp = self.create_request("sendChatAction", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos
//    /// object.
//    pub fn get_user_profile_photos(&mut self, v: String) -> Result<UserProfilePhotos, String> {
//        let resp = self.create_request("getUserProfilePhotos", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to get basic info about a file and prepare it for downloading. For the
//    /// moment, bots can download files of up to 20MB in size. On success, a File object is
//    /// returned. The file can then be downloaded via the link
//    /// https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the
//    /// response. It is guaranteed that the link will be valid for at least 1 hour. When the link
//    /// expires, a new one can be requested by calling getFile again.
//    ///
//    /// Note: This function may not preserve the original file name and MIME type. You should save
//    /// the file's MIME type and name (if available) when the File object is received.
//    pub fn get_file(&mut self, v: String) -> Result<File, String> {
//        let resp = self.create_request("getFile", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }
//
//    /// Use this method to kick a user from a group, a supergroup or a channel. In the case of
//    /// supergroups and channels, the user will not be able to return to the group on their own
//    /// using invite links, etc., unless unbanned first. The bot must be an administrator in the
//    /// chat for this to work and must have the appropriate admin rights. Returns True on success.
//    ///
//    /// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members
//    /// Are Admins’ setting is off in the target group. Otherwise members may only be removed by
//    /// the group's creator or by the member that added them.
//    pub fn kick_chat_member(&mut self, v: String) -> Result<Boolean, String> {
//        let resp = self.create_request("kickChatMember", v)?;
//        from_value(resp).map_err(|e| e.to_string())
//    }

    /// A simple method for testing your bot's auth token. Requires no parameters. Returns basic
    /// information about the bot in form of a User object.
    pub fn getMe(&mut self, v: String) -> Result<User, String> {
        let resp = self.create_request("getMe", v)?;
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
    /// To use this mode, pass Markdown in the parse_mode field when using sendMessage. Use the
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
    /// To use this mode, pass HTML in the parse_mode field when using sendMessage. The following
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
    pub fn sendMessage(&mut self, v: params::SendMessageParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to forward messages of any kind. On success, the sent Message is returned.
    pub fn forwardMessage(&mut self, v: params::ForwardMessageParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    ///Use this method to send photos. On success, the sent Message is returned.
    pub fn sendPhoto(&mut self, v: params::SendPhotoParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send audio files, if you want Telegram clients to display them in the
    /// music player. Your audio must be in the .mp3 format. On success, the sent Message is
    /// returned. Bots can currently send audio files of up to 50 MB in size, this limit may be
    /// changed in the future.
    ///
    /// For sending voice messages, use the sendVoice method instead.
    pub fn sendAudio(&mut self, v: params::SendAudioParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send general files. On success, the sent Message is returned. Bots can
    /// currently send files of any type of up to 50 MB in size, this limit may be changed in the
    /// future.
    pub fn sendDocument(&mut self, v: params::SendDocumentParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send video files, Telegram clients support mp4 videos (other formats may
    /// be sent as Document). On success, the sent Message is returned. Bots can currently send
    /// video files of up to 50 MB in size, this limit may be changed in the future.
    pub fn sendVideo(&mut self, v: params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send audio files, if you want Telegram clients to display the file as a
    /// playable voice message. For this to work, your audio must be in an .ogg file encoded with
    /// OPUS (other formats may be sent as Audio or Document). On success, the sent Message is
    /// returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be
    /// changed in the future.
    pub fn sendVoice(&mut self, v: params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long. Use
    /// this method to send video messages. On success, the sent Message is returned.
    pub fn sendVideoNote(&mut self, v: params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send a group of photos or videos as an album. On success, an array of
    /// the sent Messages is returned.
    pub fn sendMediaGroup(&mut self, v: params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send point on the map. On success, the sent Message is returned.
    pub fn sendLocation(&mut self, v: params::SendVideoParams) -> Result<Message, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to edit live location messages sent by the bot or via the bot (for inline
    /// bots). A location can be edited until its live_period expires or editing is explicitly
    /// disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent
    /// by the bot, the edited Message is returned, otherwise True is returned.
    pub fn editMessageLiveLocation(&mut self, v: params::SendVideoParams) -> Result<TrueMessage, String> {
        let resp = self.create_request(v.method(), v.to_json())?;
        from_value(resp).map_err(|e| e.to_string())
    }
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Optional	Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    message_id	Integer	Optional	Required if inline_message_id is not specified. Identifier of the sent message
//    inline_message_id	String	Optional	Required if chat_id and message_id are not specified. Identifier of the inline message
//    latitude	Float number	Yes	Latitude of new location
//    longitude	Float number	Yes	Longitude of new location
//    reply_markup	InlineKeyboardMarkup	Optional	A JSON-serialized object for a new inline keyboard.
//    stopMessageLiveLocation
//    Use this method to stop updating a live location message sent by the bot or via the bot (for inline bots) before live_period expires. On success, if the message was sent by the bot, the sent Message is returned, otherwise True is returned.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Optional	Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    message_id	Integer	Optional	Required if inline_message_id is not specified. Identifier of the sent message
//    inline_message_id	String	Optional	Required if chat_id and message_id are not specified. Identifier of the inline message
//    reply_markup	InlineKeyboardMarkup	Optional	A JSON-serialized object for a new inline keyboard.
//    sendVenue
//    Use this method to send information about a venue. On success, the sent Message is returned.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    latitude	Float number	Yes	Latitude of the venue
//    longitude	Float number	Yes	Longitude of the venue
//    title	String	Yes	Name of the venue
//    address	String	Yes	Address of the venue
//    foursquare_id	String	Optional	Foursquare identifier of the venue
//    disable_notification	Boolean	Optional	Sends the message silently. Users will receive a notification with no sound.
//    reply_to_message_id	Integer	Optional	If the message is a reply, ID of the original message
//    reply_markup	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply	Optional	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
//    sendContact
//    Use this method to send phone contacts. On success, the sent Message is returned.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    phone_number	String	Yes	Contact's phone number
//    first_name	String	Yes	Contact's first name
//    last_name	String	Optional	Contact's last name
//    disable_notification	Boolean	Optional	Sends the message silently. Users will receive a notification with no sound.
//    reply_to_message_id	Integer	Optional	If the message is a reply, ID of the original message
//    reply_markup	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply	Optional	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
//    sendChatAction
//    Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
//
//    Example: The ImageBot needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use sendChatAction with action = upload_photo. The user will see a “sending photo” status for the bot.
//
//    We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    action	String	Yes	Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_audio or upload_audio for audio files, upload_document for general files, find_location for location data, record_video_note or upload_video_note for video notes.
//    getUserProfilePhotos
//    Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
//
//    Parameters	Type	Required	Description
//    user_id	Integer	Yes	Unique identifier of the target user
//    offset	Integer	Optional	Sequential number of the first photo to be returned. By default, all photos are returned.
//    limit	Integer	Optional	Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
//    getFile
//    Use this method to get basic info about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
//
//    Parameters	Type	Required	Description
//    file_id	String	Yes	File identifier to get info about
//    Note: This function may not preserve the original file name and MIME type. You should save the file's MIME type and name (if available) when the File object is received.
//
//    kickChatMember
//    Use this method to kick a user from a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
//
//    Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group. Otherwise members may only be removed by the group's creator or by the member that added them.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
//    user_id	Integer	Yes	Unique identifier of the target user
//    until_date	Integer	Optional	Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
//    unbanChatMember
//    Use this method to unban a previously kicked user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target group or username of the target supergroup or channel (in the format @username)
//    user_id	Integer	Yes	Unique identifier of the target user
//    restrictChatMember
//    Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate admin rights. Pass True for all boolean parameters to lift restrictions from a user. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
//    user_id	Integer	Yes	Unique identifier of the target user
//    until_date	Integer	Optional	Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
//    can_send_messages	Boolean	Optional	Pass True, if the user can send text messages, contacts, locations and venues
//    can_send_media_messages	Boolean	Optional	Pass True, if the user can send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
//    can_send_other_messages	Boolean	Optional	Pass True, if the user can send animations, games, stickers and use inline bots, implies can_send_media_messages
//    can_add_web_page_previews	Boolean	Optional	Pass True, if the user may add web page previews to their messages, implies can_send_media_messages
//    promoteChatMember
//    Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Pass False for all boolean parameters to demote a user. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    user_id	Integer	Yes	Unique identifier of the target user
//    can_change_info	Boolean	Optional	Pass True, if the administrator can change chat title, photo and other settings
//    can_post_messages	Boolean	Optional	Pass True, if the administrator can create channel posts, channels only
//    can_edit_messages	Boolean	Optional	Pass True, if the administrator can edit messages of other users and can pin messages, channels only
//    can_delete_messages	Boolean	Optional	Pass True, if the administrator can delete messages of other users
//    can_invite_users	Boolean	Optional	Pass True, if the administrator can invite new users to the chat
//    can_restrict_members	Boolean	Optional	Pass True, if the administrator can restrict, ban or unban chat members
//    can_pin_messages	Boolean	Optional	Pass True, if the administrator can pin messages, supergroups only
//    can_promote_members	Boolean	Optional	Pass True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
//    exportChatInviteLink
//    Use this method to export an invite link to a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns exported invite link as String on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    setChatPhoto
//    Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
//
//    Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    photo	InputFile	Yes	New chat photo, uploaded using multipart/form-data
//    deleteChatPhoto
//    Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
//
//    Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    setChatTitle
//    Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
//
//    Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    title	String	Yes	New chat title, 1-255 characters
//    setChatDescription
//    Use this method to change the description of a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    description	String	Optional	New chat description, 0-255 characters
//    pinChatMessage
//    Use this method to pin a message in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    message_id	Integer	Yes	Identifier of a message to pin
//    disable_notification	Boolean	Optional	Pass True, if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels.
//    unpinChatMessage
//    Use this method to unpin a message in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    leaveChat
//    Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
//    getChat
//    Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
//    getChatAdministrators
//    Use this method to get a list of administrators in a chat. On success, returns an Array of ChatMember objects that contains information about all chat administrators except other bots. If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
//    getChatMembersCount
//    Use this method to get the number of members in a chat. Returns Int on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
//    getChatMember
//    Use this method to get information about a member of a chat. Returns a ChatMember object on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
//    user_id	Integer	Yes	Unique identifier of the target user
//    setChatStickerSet
//    Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
//    sticker_set_name	String	Yes	Name of the sticker set to be set as the group sticker set
//    deleteChatStickerSet
//    Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
//    answerCallbackQuery
//    Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
//
//    Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via @Botfather and accept the terms. Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
//
//    Parameters	Type	Required	Description
//    callback_query_id	String	Yes	Unique identifier for the query to be answered
//    text	String	Optional	Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
//    show_alert	Boolean	Optional	If true, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
//    url	String	Optional	URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @Botfather, specify the URL that opens your game – note that this will only work if the query comes from a callback_game button.
//
//    Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
//    cache_time	Integer	Optional	The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
//    Inline mode methods
//    Methods and objects used in the inline mode are described in the Inline mode section.
//
//    Updating messages
//    The following methods allow you to change an existing message in the message history instead of sending a new one with a result of an action. This is most useful for messages with inline keyboards using callback queries, but can also help reduce clutter in conversations with regular chat bots.
//
//    Please note, that it is currently only possible to edit messages without reply_markup or with inline keyboards.
//
//    editMessageText
//    Use this method to edit text and game messages sent by the bot or via the bot (for inline bots). On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Optional	Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    message_id	Integer	Optional	Required if inline_message_id is not specified. Identifier of the sent message
//    inline_message_id	String	Optional	Required if chat_id and message_id are not specified. Identifier of the inline message
//    text	String	Yes	New text of the message
//    parse_mode	String	Optional	Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
//    disable_web_page_preview	Boolean	Optional	Disables link previews for links in this message
//    reply_markup	InlineKeyboardMarkup	Optional	A JSON-serialized object for an inline keyboard.
//    editMessageCaption
//    Use this method to edit captions of messages sent by the bot or via the bot (for inline bots). On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Optional	Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    message_id	Integer	Optional	Required if inline_message_id is not specified. Identifier of the sent message
//    inline_message_id	String	Optional	Required if chat_id and message_id are not specified. Identifier of the inline message
//    caption	String	Optional	New caption of the message
//    reply_markup	InlineKeyboardMarkup	Optional	A JSON-serialized object for an inline keyboard.
//    editMessageReplyMarkup
//    Use this method to edit only the reply markup of messages sent by the bot or via the bot (for inline bots). On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Optional	Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    message_id	Integer	Optional	Required if inline_message_id is not specified. Identifier of the sent message
//    inline_message_id	String	Optional	Required if chat_id and message_id are not specified. Identifier of the inline message
//    reply_markup	InlineKeyboardMarkup	Optional	A JSON-serialized object for an inline keyboard.
//    deleteMessage
//    Use this method to delete a message, including service messages, with the following limitations:
//    - A message can only be deleted if it was sent less than 48 hours ago.
//    - Bots can delete outgoing messages in groups and supergroups.
//    - Bots granted can_post_messages permissions can delete outgoing messages in channels.
//    - If the bot is an administrator of a group, it can delete any message there.
//    - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
//    Returns True on success.
//
//    Parameters	Type	Required	Description
//    chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
//    message_id	Integer	Yes	Identifier of the message to delete
}
