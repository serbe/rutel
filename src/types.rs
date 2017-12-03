//#[derive(Serialize, Deserialize)]
pub struct User {
    id: u64,
    is_bot: bool,
    first_name: String,
    last_name: String,
    username: String,
    language_code: String,
}
