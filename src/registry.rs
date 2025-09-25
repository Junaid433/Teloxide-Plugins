#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use regex::Regex;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use crate::context::PluginContext;

pub struct PluginMeta {
    pub name: &'static str,
    pub commands: &'static [&'static str],
    pub prefixes: &'static [&'static str],
    pub regex: Option<&'static str>,
    pub callback_filter: Option<&'static str>,
    pub callback: fn(PluginContext) -> Pin<Box<dyn Future<Output=()> + Send>>,
}

pub static PLUGIN_REGISTRY: Lazy<Mutex<Vec<&'static PluginMeta>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

static REGEX_CACHE: Lazy<RwLock<HashMap<&'static str, Regex>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn dispatch(ctx: PluginContext) -> Result<(), teloxide::RequestError> {
    let text = ctx.message.as_ref().and_then(|m| m.text());
    let cb_data = ctx.callback_query.as_ref().and_then(|c| c.data.as_deref());

    let plugins = {
        let registry = PLUGIN_REGISTRY.lock().unwrap();
        registry.clone()
    };

    for plugin in plugins {
        if let Some(text) = text {
            for prefix in plugin.prefixes {
                for cmd in plugin.commands {
                    let full_cmd = format!("{}{}", prefix, cmd);
                    if text == full_cmd {
                        (plugin.callback)(ctx.clone()).await;
                        return Ok(());
                    }
                }
            }

            if let Some(re) = plugin.regex {
                let regex = get_or_compile_regex(re).await;
                if regex.is_match(text) {
                    (plugin.callback)(ctx.clone()).await;
                    return Ok(());
                }
            }
        }

        if let Some(cb) = cb_data {
            if let Some(filter) = plugin.callback_filter {
                if cb == filter {
                    (plugin.callback)(ctx.clone()).await;
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

pub fn register_plugin(plugin: &'static PluginMeta) {
    let mut registry = PLUGIN_REGISTRY.lock().unwrap();
    registry.push(plugin);
}

async fn get_or_compile_regex(pattern: &'static str) -> Regex {
    {
        let cache = REGEX_CACHE.read().await;
        if let Some(r) = cache.get(pattern) {
            return r.clone();
        }
    }

    let regex = Regex::new(pattern).unwrap_or_else(|_| Regex::new("(?!)").unwrap());

    let mut cache = REGEX_CACHE.write().await;
    cache.insert(pattern, regex.clone());

    regex
}
