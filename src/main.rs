use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

mod commands;
mod dialogs;
use crate::commands::command::{answer, Command};
use crate::dialogs::dialog::{start_handler, State};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     println!("{:?}", msg);
    //     bot.send_message(msg.chat.id, "Hello!").send().await?;
    //     Ok(())
    // })
    // .await;

    // Command::repl(bot.clone(), answer).await;

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
}
