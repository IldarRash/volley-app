use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::ParseMode;
use crate::domain::Event;
use crate::infrastructure::persistence::{get_db, repositories::{user_repository, event_repository}};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "get upcoming events.")]
    UpcomingEvents,
    #[command(description = "subscribe to new event notifications.")]
    Subscribe,
    #[command(description = "unsubscribe from new event notifications.")]
    Unsubscribe,
    #[command(description = "show events you are registered for.")]
    MyEvents,
}

pub async fn run() {
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let db = get_db().await.unwrap();
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::UpcomingEvents => {
            let events = event_repository::find_upcoming(&db, 5).await.unwrap();
            let mut message = String::from("<b>Upcoming Events:</b>\n");
            for event in events {
                message.push_str(&format!("- {} at {}\n", event.event_type, event.datetime));
            }
            bot.send_message(msg.chat.id, message).parse_mode(ParseMode::Html).await?
        }
        Command::Subscribe => {
            let user_id = msg.from().unwrap().id.to_string();
            // This is a simplification. In a real app, you would have a more robust way
            // to link a telegram user to your app's user.
            let user = user_repository::find_by_telegram_id(&db, &user_id).await.unwrap();
            if let Some(mut user) = user {
                user.subscribed = true;
                user_repository::update(&db, &user.id.unwrap(), &user).await.unwrap();
                bot.send_message(msg.chat.id, "You have subscribed to new event notifications.").await?
            } else {
                bot.send_message(msg.chat.id, "User not found. Please register first.").await?
            }
        }
        Command::Unsubscribe => {
            let user_id = msg.from().unwrap().id.to_string();
            let user = user_repository::find_by_telegram_id(&db, &user_id).await.unwrap();
            if let Some(mut user) = user {
                user.subscribed = false;
                user_repository::update(&db, &user.id.unwrap(), &user).await.unwrap();
                bot.send_message(msg.chat.id, "You have unsubscribed from new event notifications.").await?
            } else {
                bot.send_message(msg.chat.id, "User not found. Please register first.").await?
            }
        }
        Command::MyEvents => {
            let user_id = msg.from().unwrap().id.to_string();
            let user = user_repository::find_by_telegram_id(&db, &user_id).await.unwrap();
            if let Some(user) = user {
                let events = event_repository::find_by_user(&db, &user.id.unwrap()).await.unwrap();
                let mut message = String::from("<b>Your Events:</b>\n");
                for event in events {
                    message.push_str(&format!("- {} at {}\n", event.event_type, event.datetime));
                }
                bot.send_message(msg.chat.id, message).parse_mode(ParseMode::Html).await?
            } else {
                bot.send_message(msg.chat.id, "User not found. Please register first.").await?
            }
        }
    };

    Ok(())
}

pub async fn notify_new_event(event: &Event) {
    let bot = Bot::from_env();
    let db = get_db().await.unwrap();
    
    let users = user_repository::find_all(&db).await.unwrap();
    
    for user in users {
        if user.subscribed {
            if let Some(telegram_id) = user.telegram_id {
                let message = format!(
                    "New event created: <b>{}</b> at {}.<br>Date: {}",
                    event.event_type, event.location_id, event.datetime
                );
                bot.send_message(telegram_id, message).parse_mode(ParseMode::Html).await.ok();
            }
        }
    }
} 