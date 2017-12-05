use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    ok: bool,
    description: Option<String>,
    result: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u64,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}
