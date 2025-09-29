use teloxide::prelude::*;
use teloxide_plugins::TeloxidePlugin;

#[TeloxidePlugin(commands = ["echo"], prefixes = ["/"])]
async fn echo(bot: Bot, msg: Message) {
    if let Some(text) = msg.text() {
        bot.send_message(msg.chat.id, text).await.unwrap();
    }
}
