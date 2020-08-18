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
    assert!(u.is_bot, true);
}
