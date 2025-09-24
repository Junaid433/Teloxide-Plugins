# Teloxide Plugins 🚀

**The easiest way to create Telegram bots with Rust!**

This library makes creating Telegram bots super simple. Instead of writing complex message handling code, you just add a `#[TeloxidePlugin]` attribute above your functions, and they automatically become bot commands!

## What is this? 🤔

Imagine you want to create a Telegram bot that responds to `/ping` with "pong!". Normally, you'd need to write lots of boilerplate code to handle messages, parse commands, etc. With this library, you just write:

```rust
#[TeloxidePlugin(commands = ["ping"], prefixes = ["/"])]
async fn ping(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "pong!").await.unwrap();
}
```

That's it! The bot will automatically respond to `/ping` with "pong!".

## Prerequisites 📋

Before you start, make sure you have:

1. **Rust installed** - [Install Rust](https://rustup.rs/)
2. **A Telegram bot token** - [Get one from @BotFather](https://t.me/botfather)
3. **Basic Rust knowledge** - You should know what `async`/`await` means

## Step-by-Step Setup 🛠️

### Step 1: Create a New Project

```bash
cargo new my-telegram-bot
cd my-telegram-bot
```

### Step 2: Add Dependencies

Open `Cargo.toml` and add these dependencies:

```toml
[dependencies]
teloxide-plugins = "0.1.0"
teloxide = "0.17"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### Step 3: Create Your First Bot

Replace the contents of `src/main.rs` with this simple example:

```rust
use teloxide::prelude::*;
use teloxide_plugins::{PluginContext, dispatch, TeloxidePlugin};

#[TeloxidePlugin(commands = ["ping"], prefixes = ["/"])]
async fn ping_command(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "pong!").await.unwrap();
}

#[TeloxidePlugin(regex = ["(?i)hello"])]
async fn hello_response(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Hi there! 👋").await.unwrap();
}

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let ctx = PluginContext::new(bot.clone(), Some(msg.clone()), None);
    dispatch(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_BOT_TOKEN");
    
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
```

### Step 4: Get Your Bot Token

1. Open Telegram and search for `@BotFather`
2. Send `/newbot` to create a new bot
3. Follow the instructions to name your bot
4. Copy the token you receive
5. Replace `"YOUR_BOT_TOKEN"` in your code with the actual token

### Step 5: Run Your Bot

```bash
cargo run
```

If everything works, you should see "Starting bot..." in your terminal. Now test your bot in Telegram by sending `/ping` or any message with "hello"!

## Understanding the Magic ✨

Let's break down what's happening:

### The `#[TeloxidePlugin]` Attribute

This is the magic that makes everything work! It tells the library:
- **What commands to respond to** (`commands = ["ping"]`)
- **What prefixes to use** (`prefixes = ["/"]`)
- **When to trigger** (when someone sends `/ping`)

### Plugin Types Explained

#### 1. Command Plugins 📝
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

#### 2. Regex Plugins 🔍
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

#### 3. Callback Plugins 🔘
Handle button clicks in your bot.

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

## Common Patterns 🎯

### Echo Bot
```rust
#[TeloxidePlugin(commands = ["echo"], prefixes = ["/"])]
async fn echo(bot: Bot, msg: Message) {
    if let Some(text) = msg.text() {
        bot.send_message(msg.chat.id, text).await.unwrap();
    }
}
```

### Calculator Bot
```rust
#[TeloxidePlugin(regex = [r"^/calc\s+(\d+)\s*([+\-*/])\s*(\d+)$"])]
async fn calculator(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, "Calculator activated! 🧮").await.unwrap();
}
```

### Random Number Generator
```rust
use rand::Rng;

#[TeloxidePlugin(commands = ["random"], prefixes = ["/"])]
async fn random_number(bot: Bot, msg: Message) {
    let number = rand::thread_rng().gen_range(1..=100);
    bot.send_message(msg.chat.id, format!("Your random number: {}", number)).await.unwrap();
}
```

## Error Handling 🛡️

Sometimes things go wrong. Here's how to handle errors gracefully:

```rust
async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let ctx = PluginContext::new(bot.clone(), Some(msg.clone()), None);
    if let Err(e) = dispatch(ctx).await {
        eprintln!("Something went wrong: {}", e);
        bot.send_message(msg.chat.id, "Sorry, something went wrong! 😅").await.ok();
    }
    Ok(())
}
```

## Tips for Beginners 💡

1. **Start Simple**: Begin with basic commands like `/ping` and `/help`
2. **Test Often**: Run `cargo run` frequently to test your changes
3. **Use `println!`**: Add `println!("Debug: {:?}", msg);` to see what's happening
4. **Read Error Messages**: Rust's error messages are usually helpful
5. **Check the Examples**: Look at `examples/bot.rs` for more complex examples

## Troubleshooting 🔧

### "Bot doesn't respond"
- Check if your bot token is correct
- Make sure the bot is running (`cargo run`)
- Verify the command syntax in your plugin

### "Compilation errors"
- Make sure you have all dependencies in `Cargo.toml`
- Check that your Rust syntax is correct
- Look at the error message for hints

### "Bot responds to everything"
- Make sure your regex patterns are specific enough
- Check the order of your plugins (first match wins)

## What's Next? 🚀

Once you're comfortable with the basics, you can:

1. **Add more commands** - Create `/weather`, `/joke`, `/quote` commands
2. **Use external APIs** - Fetch data from weather services, news APIs, etc.
3. **Store data** - Use a database to remember user preferences
4. **Create interactive bots** - Use buttons and menus for better user experience
5. **Deploy your bot** - Host it on a server so it runs 24/7

## Examples to Try 🎮

Here are some fun bot ideas to get you started:

- **Weather Bot**: `/weather London` → Get weather info
- **Joke Bot**: `/joke` → Tell a random joke
- **Reminder Bot**: `/remind 5m Take out trash` → Set reminders
- **Quiz Bot**: `/quiz` → Ask random questions
- **File Converter**: Send images → Convert to different formats

## Getting Help 🆘

- **Check the examples** in the `examples/` folder
- **Read the Teloxide docs** for more advanced features
- **Ask questions** on Rust forums or Discord
- **Look at the source code** to understand how things work

## Contributing 🤝

Found a bug? Have an idea? We'd love your help!

1. Fork the repository
2. Make your changes
3. Test them thoroughly
4. Submit a pull request

## License 📄

This project is licensed under the MIT License - feel free to use it in your projects!

---

**Happy bot building! 🤖✨**