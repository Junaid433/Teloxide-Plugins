use teloxide::prelude::*;
use teloxide_plugins::{PluginContext, dispatch};
mod plugs;

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    println!("message: {:?}", msg);
    let ctx = PluginContext::new(bot.clone(), Some(msg.clone()), None);
    let _ = dispatch(ctx).await;
    Ok(())
}

async fn handle_callback_query(bot: Bot, cq: CallbackQuery) -> ResponseResult<()> {
    println!("callback query: {:?}", cq);
    let ctx = PluginContext::new(bot.clone(), None, Some(cq.clone()));
    let _ = dispatch(ctx).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Starting bot...");

    let bot = Bot::new(std::env::var("TELOXIDE_BOT_TOKEN").unwrap());


    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback_query));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}