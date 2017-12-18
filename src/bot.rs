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

    /// A simple method for testing your bot's auth token. Requires no parameters. Returns basic
    /// information about the bot in form of a User object.
    pub fn get_me(&mut self) -> Result<User, String> {
        let resp = self.create_request("getMe", String::new())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to receive incoming updates using long polling (wiki). An Array of Update
    /// objects is returned.
    ///
    /// Please note that this parameter doesn't affect updates created before the call to the
    /// getUpdates, so unwanted updates may be received for a short period of time.
    /// Notes
    /// 1. This method will not work if an outgoing webhook is set up.
    /// 2. In order to avoid getting duplicate updates, recalculate offset after each server
    /// response.
    pub fn get_updates(&mut self, v: String) -> Result<Vec<Update>, String> {
        let resp = self.create_request("getUpdates", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send text messages. On success, the sent Message is returned.
    pub fn send_message(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendMessage", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to forward messages of any kind. On success, the sent Message is returned.
    pub fn forward_message(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("forwardMessage", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send photos. On success, the sent Message is returned.
    pub fn send_photo(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendPhoto", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send audio files, if you want Telegram clients to display them in the
    /// music player. Your audio must be in the .mp3 format. On success, the sent Message is
    /// returned. Bots can currently send audio files of up to 50 MB in size, this limit may be
    /// changed in the future.
    ///
    /// For sending voice messages, use the sendVoice method instead.
    pub fn send_audio(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendAudio", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send general files. On success, the sent Message is returned. Bots can
    /// currently send files of any type of up to 50 MB in size, this limit may be changed in the
    /// future.
    pub fn send_document(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendDocument", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send video files, Telegram clients support mp4 videos (other formats may
    /// be sent as Document). On success, the sent Message is returned. Bots can currently send
    /// video files of up to 50 MB in size, this limit may be changed in the future.
    pub fn send_video(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendVideo", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send audio files, if you want Telegram clients to display the file as a
    /// playable voice message. For this to work, your audio must be in an .ogg file encoded with
    /// OPUS (other formats may be sent as Audio or Document). On success, the sent Message is
    /// returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be
    /// changed in the future.
    pub fn send_voice(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendVoice", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long.
    /// Use this method to send video messages. On success, the sent Message is returned.
    pub fn send_video_note(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendVideoNote", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send a group of photos or videos as an album. On success, an array of
    /// the sent Messages is returned.
    pub fn send_media_group(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendMediaGroup", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send point on the map. On success, the sent Message is returned.
    pub fn send_location(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendLocation", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to edit live location messages sent by the bot or via the bot (for inline
    /// bots). A location can be edited until its live_period expires or editing is explicitly
    /// disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent
    /// by the bot, the edited Message is returned, otherwise True is returned.
    pub fn edit_message_live_location(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("editMessageLiveLocation", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to stop updating a live location message sent by the bot or via the bot
    /// (for inline bots) before live_period expires. On success, if the message was sent by the
    /// bot, the sent Message is returned, otherwise True is returned.
    pub fn stop_message_live_location(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("stopMessageLiveLocation", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send information about a venue. On success, the sent Message is returned.
    pub fn send_venue(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendVenue", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send phone contacts. On success, the sent Message is returned.
    pub fn send_contact(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendContact", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method when you need to tell the user that something is happening on the bot's
    /// side. The status is set for 5 seconds or less (when a message arrives from your bot,
    /// Telegram clients clear its typing status). Returns True on success.
    ///
    /// Example: The ImageBot needs some time to process a request and upload the image. Instead
    /// of sending a text message along the lines of “Retrieving image, please wait…”, the bot may
    /// use sendChatAction with action = upload_photo. The user will see a “sending photo” status
    /// for the bot.
    ///
    /// We only recommend using this method when a response from the bot will take a noticeable
    /// amount of time to arrive.
    pub fn send_chat_action(&mut self, v: String) -> Result<Boolean, String> {
        let resp = self.create_request("sendChatAction", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos
    /// object.
    pub fn get_user_profile_photos(&mut self, v: String) -> Result<UserProfilePhotos, String> {
        let resp = self.create_request("getUserProfilePhotos", v)?;
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
    pub fn get_file(&mut self, v: String) -> Result<File, String> {
        let resp = self.create_request("getFile", v)?;
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
    pub fn kick_chat_member(&mut self, v: String) -> Result<Boolean, String> {
        let resp = self.create_request("kickChatMember", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }
}
