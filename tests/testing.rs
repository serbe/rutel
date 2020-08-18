use dotenv;
use rutel::bot;

#[test]
fn get_token() {
    let token = dotenv::var("TOKEN");
    assert!(token.is_ok());
}

#[tokio::test]
async fn test_get_me() {
    let token: String = dotenv::var("TOKEN").unwrap();
    let mut b = bot::Bot::new(&token);
    let u = b.get_me().await.unwrap();
    assert_eq!(u.is_bot, true);
}

#[tokio::test]
async fn test_send_message() {
    // let token: String = dotenv::var("TOKEN").unwrap();
    let target: String = dotenv::var("TARGET").unwrap();
    // let mut b = bot::Bot::new(&token);
    let send_message =  bot::SendMessage::new(target.into(), String::from("Hello"));
    assert_eq!(dbg!(send_message).text, String::from("Hello"));
    // let u = b.send_message(&mut bot::SendMessage::new(target.into(), String::from("Hello"))).await;
    // assert_eq!(dbg!(u).is_ok(), true);
}
