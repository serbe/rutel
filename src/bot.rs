use tokio_core::reactor::Core;
use hyper::{Client, Body, Method, Request};
use hyper_tls::{HttpsConnector};
use hyper::client::HttpConnector;
use serde_json::{from_slice, from_value, Value, Error};
use hyper::header::{ContentLength, ContentType};
use types::{Response, User};
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
    pub fn new(token: &str) -> Self {
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
        from_value(self.event_loop.run(work).unwrap())
    }

    fn create_request_with_values(mut self, method: &str, values: String) -> Result<Response, Error> {
        let uri = format!("https://api.telegram.org/bot{}/{}", self.token, method).parse().unwrap();
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(values.len() as u64));
        req.set_body(values);
//        println!("{:?}", req.body());
        let work = self.client.request(req).and_then(|res| {
            res.body().concat2().and_then(move |body| {
                let ve = body.to_vec();
                println!("{:?}", &body);
                println!("to_string_pretty {:?}", String::from_utf8_lossy(&ve).to_string());
                let v: Value = from_slice(&body).map_err(|e| {
                    println!("error {:?}", e);
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                })?;
                Ok(v)
            })
        });
        from_value(self.event_loop.run(work).unwrap())
    }

    pub fn get_me(self) -> Result<User, Error> {
        let resp = self.create_request("getMe");
        match resp {
            Ok(v) => from_value(v.result),
            Err(e) => Err(e),
        }
    }

    pub fn get_updates(self, v: String) -> Result<User, Error> {
        let resp = self.create_request_with_values("getUpdates", v);
        match resp {
            Ok(v) => from_value(v.result),
            Err(e) => Err(e),
        }
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
