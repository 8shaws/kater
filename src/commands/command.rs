use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Get to know what we provide you with😊")]
    Help,
    #[command(description = "Start with configuring your account.")]
    Start,
    #[command(description = "Get token info")]
    Token,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                "Let's take your credentials and market interest info...",
            )
            .await?
        }
        Command::Token => bot.send_message(msg.chat.id, "Token info").await?,
    };

    Ok(())
}
