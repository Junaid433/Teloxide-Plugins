use teloxide::prelude::*;
use teloxide_plugins::TeloxidePlugin;

#[TeloxidePlugin(regex = ["(?i)hello"])]
async fn hello_regex(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Hi there 👋").await.unwrap();
}
