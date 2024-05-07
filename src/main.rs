use teloxide::dispatching::UpdateFilterExt;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

mod commands;
mod dialogs;
mod keyboard;
use crate::commands::command::{answer, Command};
use crate::dialogs::dialog::{start_handler, State};
use crate::keyboard::keyboard::{callback_handler, inline_query_handler, message_handler};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
