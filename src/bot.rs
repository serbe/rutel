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
        let uri: Uri = format!("https://api.telegram.org/bot{}/{}", self.token, method).parse().unwrap();
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
                let v: Value = from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e.to_string()
                    )
                })?;
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

    /// A simple method for testing your bot's auth token. Requires no parameters. Returns basic information about the bot in form of a User object.
    pub fn get_me(&mut self) -> Result<User, String> {
        let resp = self.create_request("getMe", String::new())?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to receive incoming updates using long polling (wiki). An Array of Update objects is returned.
    ///
    /// Parameters	Type	Required	Description
    /// offset	Integer	Optional	Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will forgotten.
    /// limit	Integer	Optional	Limits the number of updates to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    /// timeout	Integer	Optional	Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    /// allowed_updates	Array of String	Optional	List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.
    ///
    /// Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time.
    /// Notes
    /// 1. This method will not work if an outgoing webhook is set up.
    /// 2. In order to avoid getting duplicate updates, recalculate offset after each server response.
    pub fn get_updates(&mut self, v: String) -> Result<Vec<Update>, String> {
        let resp = self.create_request("getUpdates", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send text messages. On success, the sent Message is returned.
    ///
    /// Parameters	Type	Required	Description
    /// chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    /// text	String	Yes	Text of the message to be sent
    /// parse_mode	String	Optional	Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    /// disable_web_page_preview	Boolean	Optional	Disables link previews for links in this message
    /// disable_notification	Boolean	Optional	Sends the message silently. Users will receive a notification with no sound.
    /// reply_to_message_id	Integer	Optional	If the message is a reply, ID of the original message
    /// reply_markup	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply	Optional	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub fn send_message(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendMessage", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to forward messages of any kind. On success, the sent Message is returned.
    ///
    /// Parameters	Type	Required	Description
    /// chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    /// from_chat_id	Integer or String	Yes	Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    /// disable_notification	Boolean	Optional	Sends the message silently. Users will receive a notification with no sound.
    /// message_id	Integer	Yes	Message identifier in the chat specified in from_chat_id
    pub fn forward_message(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("forwardMessage", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }

    /// Use this method to send photos. On success, the sent Message is returned.
    ///
    /// Parameters	Type	Required	Description
    /// chat_id	Integer or String	Yes	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    /// photo	InputFile or String	Yes	Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. More info on Sending Files »
    /// caption	String	Optional	Photo caption (may also be used when resending photos by file_id), 0-200 characters
    /// disable_notification	Boolean	Optional	Sends the message silently. Users will receive a notification with no sound.
    /// reply_to_message_id	Integer	Optional	If the message is a reply, ID of the original message
    /// reply_markup	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply	Optional	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub fn send_photo(&mut self, v: String) -> Result<Message, String> {
        let resp = self.create_request("sendPhoto", v)?;
        from_value(resp).map_err(|e| e.to_string())
    }
}
