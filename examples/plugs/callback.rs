use teloxide::prelude::*;
use teloxide_plugins::TeloxidePlugin;

#[TeloxidePlugin(callback = ["callback"])]
async fn callback(bot: Bot, cq: CallbackQuery) {
    if let Some(message) = cq.message {
        bot.send_message(message.chat().id, "Callback received!").await.unwrap();
    }
    bot.answer_callback_query(cq.id).await.unwrap();
}