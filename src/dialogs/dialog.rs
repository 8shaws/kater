use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    // ReceiveName,
    // InterestedTokens {
    //     name: String,
    // },
    // Market {
    //     name: String,
    //     tokens: Vec<String>,
    // },
}

pub async fn start_handler(bot: Bot, dialog: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Let's take your credentials and market interest info...",
    )
    .await;
    Ok(())
}
