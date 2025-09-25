# 🚀 Teloxide Plugins

<div align="center">

**The easiest way to create Telegram bots with Rust!**

[![Crates.io](https://img.shields.io/crates/v/teloxide-plugins.svg)](https://crates.io/crates/teloxide-plugins)
[![Documentation](https://docs.rs/teloxide-plugins/badge.svg)](https://docs.rs/teloxide-plugins)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

*Smart plugin system for Teloxide bots - write less code, do more!*

</div>

## ✨ What is Teloxide Plugins?

Teloxide Plugins revolutionizes Telegram bot development in Rust by providing a powerful yet simple plugin system. Instead of writing complex message handling and routing code, you just add a `#[TeloxidePlugin]` attribute above your functions, and they automatically become bot commands!

**Before (Traditional Approach):**
```rust
let handler = dptree::entry()
    .branch(Update::filter_message()
        .branch(dptree::filter(|msg: Message| msg.text() == Some("/ping".to_string()))
            .endpoint(ping_handler))
        .branch(dptree::filter(|msg: Message| msg.text() == Some("/help".to_string()))
            .endpoint(help_handler)));
```

**After (With Teloxide Plugins):**
```rust
#[TeloxidePlugin(commands = ["ping"], prefixes = ["/"])]
async fn ping(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "pong!").await.unwrap();
}
```

That's it! 🎉

## 📋 Prerequisites

- **Rust** 1.70+ - [Install Rust](https://rustup.rs/)
- **Telegram Bot Token** - [Get one from @BotFather](https://t.me/botfather)
- **Basic Rust Knowledge** - Understanding of `async`/`await`

## 🚀 Quick Start

### 1. Create Your Bot Project

```bash
cargo new my-awesome-bot
cd my-awesome-bot
```

### 2. Add Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
teloxide = "0.17"
teloxide-plugins = "0.1.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### 3. Create Your First Plugin

Replace `src/main.rs` with:

```rust
use teloxide::prelude::*;
use teloxide_plugins::{PluginContext, dispatch, TeloxidePlugin};

#[TeloxidePlugin(commands = ["ping", "p"], prefixes = ["/", "!"])]
async fn ping_handler(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "🏓 Pong!").await.unwrap();
}

#[TeloxidePlugin(regex = ["(?i)hello"])]
async fn greeting_handler(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "👋 Hi there! How can I help you?").await.unwrap();
}

#[TeloxidePlugin(commands = ["help"], prefixes = ["/"])]
async fn help_handler(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "🤖 Available commands: /ping, /help").await.unwrap();
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let ctx = PluginContext::new(bot.clone(), Some(msg.clone()), None);
    dispatch(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_BOT_TOKEN");

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));

    println!("🚀 Bot is starting... Send /ping to test!");

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
```

### 4. Get Your Bot Token

1. Message [@BotFather](https://t.me/botfather) on Telegram
2. Send `/newbot` and follow the instructions
3. Copy your bot token
4. Replace `"YOUR_BOT_TOKEN"` in the code

### 5. Run Your Bot

```bash
cargo run
```

Test it by sending `/ping` or `hello` to your bot! 🎉

## 📖 Table of Contents

- [✨ What is Teloxide Plugins?](#-what-is-teloxide-plugins)
- [📋 Prerequisites](#-prerequisites)
- [🚀 Quick Start](#-quick-start)
- [🎯 Plugin Types](#-plugin-types)
  - [Command Plugins](#command-plugins)
  - [Regex Plugins](#regex-plugins)
  - [Callback Plugins](#callback-plugins)
- [🔧 Advanced Usage](#-advanced-usage)
  - [Multiple Commands & Prefixes](#multiple-commands--prefixes)
  - [Error Handling](#error-handling)
  - [State Management](#state-management)
  - [Performance Tips](#performance-tips)
- [📚 Examples](#-examples)
  - [Echo Bot](#echo-bot)
  - [Weather Bot](#weather-bot)
  - [Calculator Bot](#calculator-bot)
  - [Interactive Menu Bot](#interactive-menu-bot)
- [🛠️ API Reference](#️-api-reference)
- [🔍 Troubleshooting](#-troubleshooting)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

## 🎯 Plugin Types

### Command Plugins 📝

Respond to specific commands like `/start`, `/help`, etc.

```rust
#[TeloxidePlugin(commands = ["start", "help"], prefixes = ["/", "!"])]
async fn help_command(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Welcome! Use /ping to test me!").await.unwrap();
}
```

**What this does:**
- Responds to `/start` OR `/help`
- Also responds to `!start` OR `!help`
- Sends a welcome message

### Regex Plugins 🔍

Respond to messages that match a pattern.

```rust
#[TeloxidePlugin(regex = ["(?i)good morning"])]
async fn morning_greeting(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Good morning! ☀️").await.unwrap();
}
```

**What this does:**
- `(?i)` means case-insensitive
- Responds to "good morning", "Good Morning", "GOOD MORNING", etc.

### Callback Plugins 🔘

Handle button clicks and inline keyboard interactions.

```rust
#[TeloxidePlugin(commands = ["menu"], prefixes = ["/"])]
async fn show_menu(bot: Bot, msg: Message) {
    let button = InlineKeyboardButton::new(
        "Click me!",
        InlineKeyboardButtonKind::CallbackData("button_clicked".to_string())
    );
    let keyboard = InlineKeyboardMarkup::new(vec![vec![button]]);

    bot.send_message(msg.chat.id, "Choose an option:")
        .reply_markup(keyboard)
        .await
        .unwrap();
}

#[TeloxidePlugin(callback = ["button_clicked"])]
async fn handle_button_click(bot: Bot, cq: CallbackQuery) {
    if let Some(message) = cq.message {
        bot.send_message(message.chat().id, "Button was clicked! 🎉").await.unwrap();
        bot.answer_callback_query(cq.id).await.unwrap();
    }
}
```

## 🔧 Advanced Usage

### Multiple Commands & Prefixes

One plugin can handle multiple commands with different prefixes:

```rust
#[TeloxidePlugin(commands = ["start", "help", "h"], prefixes = ["/", "!", "."])]
async fn universal_help(bot: Bot, msg: Message) {
    bot.send_message(
        msg.chat.id,
        "🤖 Available: /start, /help, !start, !help, .start, .help"
    ).await.unwrap();
}
```

### Error Handling

Handle errors gracefully in your plugins:

```rust
#[TeloxidePlugin(commands = ["error_test"], prefixes = ["/"])]
async fn error_example(bot: Bot, msg: Message) {
    match bot.send_message(msg.chat.id, "This might fail!").await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to send message: {:?}", e);
            let _ = bot.send_message(msg.chat.id, "❌ Sorry, something went wrong!").await;
        }
    }
}
```

### State Management

For stateful bots, you can use static variables or dependency injection:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[TeloxidePlugin(commands = ["count"], prefixes = ["/"])]
async fn counter_bot(bot: Bot, msg: Message) {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    bot.send_message(msg.chat.id, format!("Count: {}", count + 1))
        .await.unwrap();
}
```

## 📚 Examples

### Echo Bot

```rust
#[TeloxidePlugin(commands = ["echo"], prefixes = ["/"])]
async fn echo_bot(bot: Bot, msg: Message) {
    if let Some(text) = msg.text() {
        bot.send_message(msg.chat.id, text).await.unwrap();
    }
}
```

### Weather Bot (External API)

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    weather: Vec<WeatherInfo>,
    main: MainInfo,
}

#[derive(Deserialize)]
struct WeatherInfo {
    main: String,
    description: String,
}

#[derive(Deserialize)]
struct MainInfo {
    temp: f64,
    humidity: u32,
}

#[TeloxidePlugin(commands = ["weather"], prefixes = ["/"])]
async fn weather_bot(bot: Bot, msg: Message) {
    if let Some(city) = msg.text().unwrap().strip_prefix("/weather ") {
        match fetch_weather(city).await {
            Ok(weather) => {
                let response = format!(
                    "🌤️ Weather in {}: {} ({}°C, {}% humidity)",
                    city, weather.weather[0].description,
                    weather.main.temp, weather.main.humidity
                );
                bot.send_message(msg.chat.id, response).await.unwrap();
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "❌ Failed to fetch weather").await.unwrap();
            }
        }
    }
}
```

### Interactive Menu Bot

```rust
#[TeloxidePlugin(commands = ["menu"], prefixes = ["/"])]
async fn show_menu(bot: Bot, msg: Message) {
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::new("Option 1", InlineKeyboardButtonKind::CallbackData("opt1".to_string())),
            InlineKeyboardButton::new("Option 2", InlineKeyboardButtonKind::CallbackData("opt2".to_string())),
        ],
        vec![
            InlineKeyboardButton::new("Option 3", InlineKeyboardButtonKind::CallbackData("opt3".to_string())),
        ]
    ]);

    bot.send_message(msg.chat.id, "Choose an option:")
        .reply_markup(keyboard)
        .await.unwrap();
}

#[TeloxidePlugin(callback = ["opt1", "opt2", "opt3"])]
async fn handle_menu_selection(bot: Bot, cq: CallbackQuery) {
    if let Some(data) = &cq.data {
        let response = match data.as_str() {
            "opt1" => "You selected Option 1! 🎯",
            "opt2" => "You selected Option 2! 🎯",
            "opt3" => "You selected Option 3! 🎯",
            _ => "Unknown option"
        };

        if let Some(message) = cq.message {
            bot.send_message(message.chat().id, response).await.unwrap();
        }
        bot.answer_callback_query(cq.id).await.unwrap();
    }
}
```

## 🛠️ API Reference

### `#[TeloxidePlugin]` Attributes

| Attribute | Type | Description | Example |
|-----------|------|-------------|---------|
| `commands` | `Vec<&str>` | Command names to respond to | `["ping", "start"]` |
| `prefixes` | `Vec<&str>` | Command prefixes | `["/", "!"]` |
| `regex` | `&str` | Regex pattern for matching | `"(?i)hello"` |
| `callback` | `&str` | Callback data filter | `"button_clicked"` |

### Core Types

- **`PluginContext`**: Contains bot instance, message, and callback query
- **`dispatch()`**: Main function that routes messages to appropriate plugins
- **`PluginMeta`**: Metadata structure for plugin registration

## ⚡ Performance & Architecture

### Runtime-Free Plugin Registration
Plugins are registered during static initialization, before the tokio runtime starts:

```rust
#[ctor::ctor]
fn plugin_constructor() {
    register_plugin(&plugin_metadata);
}
```

### Zero-Blocking Runtime
Once the bot is running, all operations are fully async:

- ✅ **Plugin registration**: Static initialization (microseconds)
- ✅ **Message dispatch**: Fully async with `await` points
- ✅ **Plugin execution**: Each plugin runs independently async
- ✅ **Regex compilation**: Cached with concurrent read access

### Production Benefits
- **Startup**: ~1 microsecond per plugin registration
- **Runtime**: Zero blocking operations, unlimited concurrent message processing
- **Scalability**: Handles thousands of concurrent messages efficiently
- **Reliability**: No runtime dependency during initialization

## 🔍 Troubleshooting

### Common Issues

#### Bot Doesn't Respond
- ✅ **Check bot token**: Verify your token is correct and active
- ✅ **Check permissions**: Ensure bot can send messages in the chat
- ✅ **Verify plugin syntax**: Make sure `#[TeloxidePlugin]` attributes are correct
- ✅ **Check bot is running**: Run `cargo run` and look for startup messages

#### Compilation Errors
- ✅ **Update dependencies**: Ensure all versions in `Cargo.toml` are compatible
- ✅ **Check Rust version**: Requires Rust 1.70+
- ✅ **Import statements**: Verify all necessary imports are present

#### Regex Not Matching
- ✅ **Test pattern**: Use online regex testers to validate your patterns
- ✅ **Case sensitivity**: Use `(?i)` flag for case-insensitive matching
- ✅ **Anchors**: Add `^` and `$` for exact matches

#### Performance Issues
- ✅ **Regex compilation**: Patterns are cached automatically
- ✅ **Plugin count**: Too many plugins can slow down dispatch
- ✅ **Message frequency**: Consider rate limiting for high-traffic bots

### Debug Mode

Enable debug logging to see what's happening:

```rust
#[TeloxidePlugin(commands = ["debug"], prefixes = ["/"])]
async fn debug_handler(bot: Bot, msg: Message) {
    println!("Debug - Message: {:?}", msg);
    println!("Debug - Text: {:?}", msg.text());
    println!("Debug - Chat ID: {:?}", msg.chat.id);

    bot.send_message(msg.chat.id, "Debug info logged to console").await.unwrap();
}
```

## 🤝 Contributing

We welcome contributions! Here's how to get involved:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup

```bash
git clone https://github.com/Junaid433/teloxide-plugins.git
cd teloxide-plugins

cargo test

cargo run --example bot
```

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**Happy bot building! 🤖✨**

*Made with ❤️ for the Rust community*

[⭐ Star us on GitHub](https://github.com/Junaid433/teloxide-plugins) • [🐛 Report Issues](https://github.com/Junaid433/teloxide-plugins/issues) • [💬 Discussions](https://github.com/Junaid433/teloxide-plugins/discussions)

</div>