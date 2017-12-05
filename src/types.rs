use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    ok: bool,
    result: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u64,
    is_bot: bool,
    first_name: String,
    last_name: String,
    username: String,
    language_code: String,
}
