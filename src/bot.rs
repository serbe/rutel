use tokio_core::reactor::Core;
use hyper::{Client, Body, Method, Request, Uri};
use hyper::client::HttpConnector;
use hyper::header::{ContentLength, ContentType};
use types::{User};

#[derive(Debug)]
pub struct Bot {
    token: String,
    core: Core,
    client: Client<HttpConnector, Body>,
//    user: Option<User>
}

impl Bot {
    pub fn new(token: &str) -> Self {
        let mut core = Core::new().unwrap();
        let client: Client<HttpConnector, Body> = Client::new(&core.handle());
        Bot {
            token: token.to_string(),
            core: core,
            client: client,
        }
    }

    pub fn get_me(self) -> () {
        let req = self.create_request("getMe", "");
        println!("{:?}", req);
    }

    fn create_request(self, method: &str, values: &str) -> Request<Body> {
//        let json = r#"{"library":"hyper"}"#;
        let uri: Uri = format!("https://api.telegram.org/bot{}/{}", self.token, method).parse().unwrap();
        let mut req: Request<Body> = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(values.len() as u64));
        req.set_body(values.to_string());
        req
    }
//    fullURL := urlAPI + bot.Token + "/" + method
//    resp, err := bot.Client.PostForm(fullURL, values)
//    if err != nil {
//    errLog("createResponse bot.Client.PostForm", err)
//    return Response{}, err
//    }
//
//    defer func() {
//    err = resp.Body.Close()
//    if err != nil {
//    errLog("createResponse resp.Body.Close", err)
//    }
//    }()
//
//    if resp.StatusCode == http.StatusForbidden {
//    err = ErrForbiddenHTTP
//    errLog("createResponse http.StatusForbidden", err)
//    return Response{}, err
//    }
//
//    body, err := ioutil.ReadAll(resp.Body)
//    if err != nil {
//    errLog("createResponse ioutil.ReadAll", err)
//    return Response{}, err
//    }
//
//    debugLog("debugLog " + method + " " + string(body))
//
//    var response Response
//
//    err = json.Unmarshal(body, &response)
//    if err != nil {
//    errLog("createResponse json.Unmarshal", err)
//    return Response{}, err
//    }
//
//    if !response.Ok {
//    err = errors.New(response.Description)
//    errLog("createResponse !response.Ok", err)
//    return response, err
//    }
//
//    return response, nil
//    }
//    pub fn GetMe(&self) {
//
//    }

}
