use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Help,
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
    // dialog.update(State::Start).await?;
    Ok(())
}

pub async fn dialog_handler(bot: Bot) -> HandlerResult {
    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start_handler)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
    Ok(())
}
