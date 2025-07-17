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
}

pub async fn run() {
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::UpcomingEvents => {
            let db = get_db().await.unwrap();
            let events = event_repository::find_upcoming(&db, 5).await.unwrap();
            let mut message = String::from("<b>Upcoming Events:</b>\n");
            for event in events {
                message.push_str(&format!("- {} at {}\n", event.event_type, event.datetime));
            }
            bot.send_message(msg.chat.id, message).parse_mode(ParseMode::Html).await?
        }
    };

    Ok(())
}

pub async fn notify_new_event(event: &Event) {
    let bot = Bot::from_env();
    let db = get_db().await.unwrap();
    
    let users = user_repository::find_all(&db).await.unwrap();
    
    for user in users {
        if let Some(telegram_id) = user.telegram_id {
            let message = format!(
                "New event created: <b>{}</b> at {}.<br>Date: {}",
                event.event_type, event.location_id, event.datetime
            );
            bot.send_message(telegram_id, message).parse_mode(ParseMode::Html).await.ok();
        }
    }
} 