use std::error::Error;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    payloads::SendMessageSetters,
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me,
    },
    utils::command::BotCommands,
};

use crate::commands::command::Command;
use crate::dialogs::dialog::{start_handler, State};

pub enum Token {
    Btc,
    Eth,
    Sol,
}

impl Token {
    fn to_string(token: &str) -> Option<Self> {
        match token.to_uppercase().as_str() {
            "BTC" => Some(Token::Btc),
            "ETH" => Some(Token::Eth),
            "SOL" => Some(Token::Sol),
            _ => None,
        }
    }
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Start) => {
                let keyboard = make_keyboard();
                bot.send_message(msg.chat.id, "Debian versions:")
                    .reply_markup(keyboard)
                    .await?;
            }
            Ok(Command::Token) => {
                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() < 2 {
                    bot.send_message(msg.chat.id, "Please provide a token")
                        .await?;
                    return Ok(());
                }
                let token = match Token::to_string(parts[1]) {
                    Some(token) => token,
                    None => {
                        bot.send_message(msg.chat.id, "Invalid token").await?;
                        return Ok(());
                    }
                };
                bot.send_message(msg.chat.id, "Token info").await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}

pub async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let choose_debian_version = InlineQueryResultArticle::new(
        "0",
        "Chose debian version",
        InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
    )
    .reply_markup(make_keyboard());

    bot.answer_inline_query(q.id, vec![choose_debian_version.into()])
        .await?;

    Ok(())
}

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(version) = q.data {
        let text = format!("You chose: {version}");

        bot.answer_callback_query(q.id).await?;

        if let Some(Message { id, chat, .. }) = q.message {
            bot.edit_message_text(chat.id, id, text).await?;
        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }

        log::info!("You chose: {}", version);
    }

    Ok(())
}

pub fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let debian_versions = [
        "Buzz", "Rex", "Bo", "Hamm", "Slink", "Potato", "Woody", "Sarge", "Etch", "Lenny",
        "Squeeze", "Wheezy", "Jessie", "Stretch", "Buster", "Bullseye",
    ];

    for versions in debian_versions.chunks(3) {
        let row = versions
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
