// use dotenv;
// use rutel::bot;
use rutel::types::ChatID;

// #[test]
// fn get_token() {
//     let token = dotenv::var("TOKEN");
//     assert!(token.is_ok());
// }

// #[tokio::test]
// async fn test_get_me() {
//     let token: String = dotenv::var("TOKEN").unwrap();
//     let mut b = bot::Bot::new(&token);
//     let u = b.get_me().await.unwrap();
//     assert_eq!(u.is_bot, true);
// }

#[test]
fn chat_id() {
    let i: ChatID = ChatID::from(-1001102759484i64);
    let c = match i {
        ChatID::String(_) => 0,
        ChatID::Integer(n) => n,
    };
    assert_eq!(c, -1001102759484i64);
}