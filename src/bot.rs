use tokio_core::reactor::Core;
use hyper::{Client, Body, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use serde_json::{from_slice, from_value, Value};
use hyper::header::{ContentLength, ContentType};
use types::{Response, User, Update, Message};
use std::io;
use futures::future::Future;
use futures::Stream;

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
            .parse().unwrap();
        uri
    }

    fn create_request(&mut self, method: &'static str, values: String) -> Result<Value, String> {
        let uri = self.build_uri(method);
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(values.len() as u64));
        req.set_body(values);
        let work = self.client
            .request(req)
            .and_then(|res| res.body()
                .concat2()
                .and_then(move |body| {
                    let v: Value = from_slice(&body).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            e.to_string()
                        )
                    })?;
                    Ok(v)
                })
            );
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
    /// 2. In order to avoid getting duplicate updates, recalculate offset after each server response.
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
}
