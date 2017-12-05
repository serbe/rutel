//use tokio_core::reactor::Core;
//use hyper::{Client, Body, Method, Request, Uri};
//use hyper::client::HttpConnector;
//use hyper::header::{ContentLength, ContentType};
use types::{Response};

use reqwest::{get, Client};

//use reqwest;

use std::io::Read;
use serde_json::{from_str};

#[derive(Debug)]
pub struct Bot {
    token: String,
//    core: Core,
//    client: Client<HttpConnector, Body>,
//    user: Option<User>
}

//#[derive(Debug)]
//enum ResponseError {
//    Serde(serde_json::Error),
//    Reqwest(reqwest::Error),
//}

impl Bot {
    pub fn new(token: &str) -> Self {
//        let mut core = Core::new().unwrap();
//        let client: Client<HttpConnector, Body> = Client::new(&core.handle());
        Bot {
            token: token.to_string(),
//            core: core,
//            client: client,
        }
    }

    pub fn get_me(self) -> () {
        let req = self.get_response("getMe");
        println!("{:?}", req.unwrap());
    }

    fn get_response(self, method: &str) -> Result<Response, String> {
        let uri = format!("https://api.telegram.org/bot{}/{}", self.token, method);
        let mut resp = try!(get(&uri).map_err(|e| e.to_string()));
        let mut content = String::new();
        resp.read_to_string(&mut content);
        let r = try!(from_str(&content).map_err(|e| e.to_string()));
        Ok(r)
    }

    fn get_response_with_values(self, method: &str, values: &str) -> Result<Response, String> {
        let uri = format!("https://api.telegram.org/bot{}/{}", self.token, method);
        let client = Client::new();
        let mut resp = try!(client.post(&uri).json(&values).send().map_err(|e| e.to_string()));
        let mut content = String::new();
        resp.read_to_string(&mut content);
        let r = try!(from_str(&content).map_err(|e| e.to_string()));
        Ok(r)
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
