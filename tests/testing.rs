use dotenv;
use rutel::bot;
use rutel::types::ChatID;

#[tokio::test]
async fn test_get_me() {
    if let Ok(token) = dotenv::var("TG_TOKEN") {
        let mut b = bot::Bot::new(&token);
        let u = b.get_me(&bot::GetMe {}).await.unwrap();
        assert_eq!(u.is_bot, true);
    }
}

#[test]
fn chat_id() {
    let i: ChatID = ChatID::from(-1001102759484i64);
    let c = match i {
        ChatID::String(_) => 0,
        ChatID::Integer(n) => n,
    };
    assert_eq!(c, -1001102759484i64);
}

#[tokio::test]
async fn send_message() {
    if let Ok(token) = dotenv::var("TG_TOKEN") {
        if let Ok(target) = dotenv::var("TARGET") {
            let mut b = bot::Bot::new(&token);
            let msg = b
                .send_message(&bot::SendMessage::new(
                    ChatID::from(target),
                    "Hi".to_string(),
                ))
                .await
                .unwrap();
            assert_eq!(msg.via_bot.is_some(), false);
        }
    }
}
