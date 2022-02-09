use dotenv;
use rutel::bot;
use rutel::types::{ChatID, InputFileString};

fn get_tt() -> Option<(bot::Bot, String)> {
    if let (Ok(token), Ok(target)) = (dotenv::var("TG_TOKEN"), dotenv::var("TARGET")) {
        return Some((bot::Bot::new(&token), target));
    } else {
        return None;
    }
}

#[tokio::test]
async fn test_get_me() {
    if let Some((mut bot, _target)) = get_tt() {
        let u = bot.get_me(&bot::GetMe {}).await.unwrap();
        assert!(u.is_bot);
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
async fn test_get_updates() {
    if let Some((mut bot, _target)) = get_tt() {
        let updates = bot.get_updates(&bot::GetUpdates::new()).await;
        dbg!(&updates);
        assert!(updates.is_ok());
    }
}

#[tokio::test]
async fn test_send_message() {
    if let Some((mut bot, target)) = get_tt() {
        let msg = bot
            .send_message(&bot::SendMessage::new(
                ChatID::from(&target),
                "Hi".to_string(),
            ))
            .await
            .unwrap();
        dbg!(&msg);
        assert!(msg.text.is_some());
    }
}

#[tokio::test]
async fn test_forward_message() {
    if let Some((mut bot, target)) = get_tt() {
        if let Ok(msg_id) = dotenv::var("MESSAGE_ID") {
            let msg = bot
                .forward_message(&bot::ForwardMessage::new(
                    ChatID::from(&target),
                    ChatID::from(&target),
                    msg_id.parse::<i64>().unwrap(),
                ))
                .await
                .unwrap();
            assert!(msg.text.is_some());
        }
    }
}

#[tokio::test]
async fn test_copy_message() {
    if let Some((mut bot, target)) = get_tt() {
        if let Ok(msg_id) = dotenv::var("MESSAGE_ID") {
            let message_id = bot
                .copy_message(&bot::CopyMessage::new(
                    ChatID::from(&target),
                    ChatID::from(&target),
                    msg_id.parse::<i64>().unwrap(),
                ))
                .await
                .unwrap();
            assert!(message_id.message_id > 0);
        }
    }
}

#[tokio::test]
async fn test_send_photo() {
    if let Some((mut bot, target)) = get_tt() {
        if let Ok(photo_id) = dotenv::var("PHOTO_ID") {
            let message = bot
                .send_photo(&bot::SendPhoto::new(
                    ChatID::from(&target),
                    InputFileString::String(photo_id),
                ))
                .await
                .unwrap();
            assert!(message.photo.is_some());
        }
    }
}

#[tokio::test]
async fn test_send_audio() {
    if let Some((mut bot, target)) = get_tt() {
        if let Ok(audio_id) = dotenv::var("AUDIO_ID") {
            let message = bot
                .send_audio(&bot::SendAudio::new(
                    ChatID::from(&target),
                    InputFileString::String(audio_id),
                ))
                .await
                .unwrap();
            assert!(message.audio.is_some());
        }
    }
}

// -----------------


#[tokio::test]
async fn test_send_poll() {
    if let Some((mut bot, target)) = get_tt() {
        let message = bot
            .send_poll(&bot::SendPoll::new(
                ChatID::from(&target),
                "Question".to_string(),
                vec!["one".to_string(), "two".to_string()],
            ))
            .await
            .unwrap();
        assert!(message.poll.is_some());
    }
}

#[tokio::test]
async fn test_send_dice() {
    if let Some((mut bot, target)) = get_tt() {
        let message = bot
            .send_dice(&bot::SendDice::new(ChatID::from(&target)).emoji(Some("🎰".to_string())))
            .await
            .unwrap();
        assert!(message.dice.is_some());
    }
}


// -----------------


#[tokio::test]
async fn test_get_chat() {
    if let Some((mut bot, target)) = get_tt() {
        let chat = bot
            .get_chat(&bot::GetChat::new(ChatID::from(&target)))
            .await
            .unwrap();
            dbg!(&chat);
        assert_eq!(chat.id, target.parse::<i64>().unwrap());
    }
}