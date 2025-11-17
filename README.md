# 🚀 Teloxide Plugins

<div align="center">

The easiest way to create Telegram bots with Rust!

[![Crates.io](https://img.shields.io/crates/v/teloxide-plugins.svg)](https://crates.io/crates/teloxide-plugins)
[![Documentation](https://docs.rs/teloxide-plugins/badge.svg)](https://docs.rs/teloxide-plugins)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

*Smart plugin system for Teloxide bots - write less code, do more!*

</div>

## What is this?

It's a macro-based plugin system for teloxide. Instead of manually wiring up dispatch trees, you slap a `#[TeloxidePlugin]` attribute on your functions and the macro handles the routing.

The old way:

```rust
let handler = dptree::entry()
    .branch(Update::filter_message()
        .branch(dptree::filter(|msg: Message| msg.text() == Some("/ping".to_string()))
            .endpoint(ping_handler))
        .branch(dptree::filter(|msg: Message| msg.text() == Some("/help".to_string()))
            .endpoint(help_handler)));
```

The new way:

```rust
#[TeloxidePlugin(commands = ["ping"], prefixes = ["/"])]
async fn ping(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "pong!").await.unwrap();
}
```

That's the basic idea.

## Requirements

- Rust 1.70 or newer
- A Telegram bot token (get one from @BotFather)
- Basic familiarity with async Rust

## Quick Start

### 1. Create a new project

```bash
cargo new my-bot
cd my-bot
```

### 2. Add dependencies

Your `Cargo.toml` should look something like:

```toml
[dependencies]
teloxide = "0.17"
teloxide-plugins = "0.1.4"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### 3. Write your first bot

Replace `src/main.rs` with this:

```rust
use teloxide::prelude::*;
use teloxide_plugins::{PluginContext, dispatch, TeloxidePlugin};

#[TeloxidePlugin(commands = ["ping", "p"], prefixes = ["/", "!"])]
async fn ping_handler(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "🏓 Pong!").await.unwrap();
}

#[TeloxidePlugin(regex = ["(?i)hello"])]
async fn greeting_handler(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Hey!").await.unwrap();
}

#[TeloxidePlugin(commands = ["help"], prefixes = ["/"])]
async fn help_handler(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Commands: /ping, /help").await.unwrap();
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let ctx = PluginContext::new(bot.clone(), Some(msg.clone()), None);
    dispatch(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_BOT_TOKEN_HERE"); 
    
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));

    println!("Bot is running... try sending /ping");

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
```

### 4. Run it

```bash
cargo run
```

Then message your bot with `/ping` or just "hello".

## Plugin Types

### Command Plugins

For commands like `/start` or `/help`:

```rust
#[TeloxidePlugin(commands = ["start", "help"], prefixes = ["/", "!"])]
async fn help_command(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Welcome!").await.unwrap();
}
```

You can pass multiple commands and prefixes. The handler will respond to any combination.

### Regex Plugins

For pattern matching:

```rust
#[TeloxidePlugin(regex = ["(?i)good morning"])]
async fn morning_greeting(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Morning! ☕").await.unwrap();
}
```

The `(?i)` flag makes it case-insensitive. You can use full regex features here, but keep in mind it'll run on every message, so don't go too crazy with complex patterns.

### Callback Plugins

For handling inline button clicks:

```rust
#[TeloxidePlugin(commands = ["menu"], prefixes = ["/"])]
async fn show_menu(bot: Bot, msg: Message) {
    let button = InlineKeyboardButton::new(
        "Click me",
        InlineKeyboardButtonKind::CallbackData("btn_click".to_string())
    );
    let keyboard = InlineKeyboardMarkup::new(vec![vec![button]]);
    
    bot.send_message(msg.chat.id, "Pick something:")
        .reply_markup(keyboard)
        .await
        .unwrap();
}

#[TeloxidePlugin(callback = ["btn_click"])]
async fn handle_click(bot: Bot, cq: CallbackQuery) {
    if let Some(msg) = cq.message {
        bot.send_message(msg.chat().id, "You clicked it!").await.unwrap();
        bot.answer_callback_query(cq.id).await.unwrap();
    }
}
```

## Advanced Usage

### Error Handling

The macro doesn't magically handle errors for you. You'll still need to deal with them in your handlers:

```rust
#[TeloxidePlugin(commands = ["might_fail"], prefixes = ["/"])]
async fn error_example(bot: Bot, msg: Message) {
    match bot.send_message(msg.chat.id, "Trying...").await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Send failed: {}", e);
        }
    }
}
```

### State Management

There's no built-in state management yet. For simple counters, you can use statics:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNT: AtomicU32 = AtomicU32::new(0);

#[TeloxidePlugin(commands = ["count"], prefixes = ["/"])]
async fn counter(bot: Bot, msg: Message) {
    let current = COUNT.fetch_add(1, Ordering::SeqCst);
    bot.send_message(msg.chat.id, format!("Count: {}", current + 1))
        .await.unwrap();
}
```

For anything more complex, you'll need to roll your own solution or wait for a future version.

### Performance

Plugin registration happens at startup, not runtime. The regex patterns are compiled once and cached. For bots handling tons of messages, the dispatch overhead is minimal - it's basically a hashmap lookup and a regex match against cached patterns.

If you have hundreds of plugins with complex regexes, yeah, maybe reconsider your bot design.

## Examples

Check the `examples/` directory in the repo for full working bots:

- `echo.rs` - Simple echo functionality
- `weather.rs` - Hitting an external API (needs an API key)
- `menu.rs` - Interactive menus with callbacks

The weather example looks like this in practice:

```rust
#[TeloxidePlugin(commands = ["weather"], prefixes = ["/"])]
async fn weather(bot: Bot, msg: Message) {
    if let Some(text) = msg.text() {
        if let Some(city) = text.strip_prefix("/weather ") {
            match get_weather_for_city(city).await {
                Ok(w) => {
                    bot.send_message(msg.chat.id, format!("It's {}°C in {}", w.temp, city))
                        .await.unwrap();
                }
                Err(_) => {
                    bot.send_message(msg.chat.id, "Couldn't fetch weather")
                        .await.unwrap();
                }
            }
        }
    }
}
```

You'll need to implement `get_weather_for_city()` yourself - this crate doesn't include HTTP clients.

## API Reference

The `#[TeloxidePlugin]` macro accepts these attributes:

| Attribute | What it does | Example |
|-----------|--------------|---------|
| `commands` | List of command names | `["ping", "start"]` |
| `prefixes` | Command prefixes | `["/", "!"]` |
| `regex` | Regex patterns to match | `["(?i)hi"]` |
| `callback` | Callback data strings | `["btn1"]` |

You can combine multiple attributes on the same function, though it's usually cleaner to keep them separate.

## Troubleshooting

**Bot doesn't respond?**

- Double-check your token
- Make sure you're actually calling `dispatch()` in your message handler
- Run with `RUST_LOG=debug` to see what's happening

**Compilation errors?**

- This macro relies on `teloxide` 0.17+. Older versions won't work.
- Make sure you've imported the macro: `use teloxide_plugins::TeloxidePlugin;`

**Regex not matching?**

Test your pattern on [regex101.com](https://regex101.com) first. Remember that `(?i)` is your friend for case-insensitive matches.

## Contributing

This is a side project, so PRs are welcome but might take a bit to review. Some areas that need work:

- Better error messages from the macro
- State management helpers
- More test coverage

To get started:

```bash
git clone https://github.com/Junaid433/teloxide-plugins.git
cd teloxide-plugins
cargo test
```

## License

MIT - see LICENSE file for details.

---

*Built because I got tired of copy-pasting dispatch trees*

[GitHub](https://github.com/Junaid433/teloxide-plugins) - [Issues](https://github.com/Junaid433/teloxide-plugins/issues)