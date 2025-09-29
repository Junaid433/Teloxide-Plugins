use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, Message};

#[derive(Clone)]
pub struct PluginContext {
    pub bot: Bot,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
}

impl PluginContext {
    pub fn new(bot: Bot, message: Option<Message>, callback_query: Option<CallbackQuery>) -> Self {
        Self {
            bot,
            message,
            callback_query,
        }
    }
}
