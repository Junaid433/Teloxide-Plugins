use teloxide::prelude::*;
use teloxide_plugins::TeloxidePlugin;

#[TeloxidePlugin(commands = ["ping", "p"], prefixes = ["/", "!"])]
async fn ping(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "pong!").await.unwrap();
}
