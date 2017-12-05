use tokio_core::reactor::Core;
use hyper::{Client, Body};
use hyper_tls::{HttpsConnector};
use hyper::client::HttpConnector;
use serde_json::{from_slice, from_value, Value, Error};
//use hyper::header::{ContentLength, ContentType};
use types::{Response};
use std::io;
use futures::future::Future;
use futures::Stream;

#[derive(Debug)]
pub struct Bot {
    token: String,
    core: Core,
    client: Client<HttpsConnector<HttpConnector>, Body>,
//    user: Option<User>
}

impl Bot {
    pub fn new(token: &str) -> Self {
        let core = Core::new().unwrap();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap())
            .build(&core.handle());
        Bot {
            token: token.to_string(),
            core: core,
            client: client,
        }
    }

    pub fn get_me(self) -> () {
        let req = self.create_request("getMe");
        println!("{:?}", req);
    }

    fn create_request(mut self, method: &str) -> Result<Response, Error> {
        let uri = format!("https://api.telegram.org/bot{}/{}", self.token, method).parse().unwrap();
        let work = self.client.get(uri).and_then(|res| {
            res.body().concat2().and_then(move |body| {
                let v: Value = from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                })?;
                Ok(v)
            })
        });
        from_value(self.core.run(work).unwrap())
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
