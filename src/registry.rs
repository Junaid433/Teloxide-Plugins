#![allow(non_upper_case_globals)]

use crate::context::PluginContext;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Mutex, RwLock as StdRwLock};
use tokio::sync::RwLock as AsyncRwLock;

pub struct PluginMeta {
    pub name: &'static str,
    pub commands: &'static [&'static str],
    pub prefixes: &'static [&'static str],
    pub regex: Option<&'static str>,
    pub callback_filter: Option<&'static str>,
    pub callback: fn(PluginContext) -> Pin<Box<dyn Future<Output = ()> + Send>>,
}

pub static PLUGIN_REGISTRY: Lazy<Mutex<Vec<&'static PluginMeta>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

static REGEX_CACHE: Lazy<AsyncRwLock<HashMap<&'static str, Regex>>> =
    Lazy::new(|| AsyncRwLock::new(HashMap::new()));

static COMMAND_MAP: Lazy<StdRwLock<HashMap<String, &'static PluginMeta>>> =
    Lazy::new(|| StdRwLock::new(HashMap::new()));

fn find_command_plugin(text: &str) -> Option<&'static PluginMeta> {
    let map = COMMAND_MAP.read().unwrap();
    map.get(text).copied()
}

pub async fn dispatch(ctx: PluginContext) -> Result<(), teloxide::RequestError> {
    let text = ctx.message.as_ref().and_then(|m| m.text());
    let cb_data = ctx.callback_query.as_ref().and_then(|c| c.data.as_deref());

    if let Some(text) = text {
        if let Some(plugin) = find_command_plugin(text) {
            (plugin.callback)(ctx.clone()).await;
            return Ok(());
        }
    }

    let plugins = {
        let registry = PLUGIN_REGISTRY.lock().unwrap();
        registry.clone()
    };

    for plugin in plugins {
        if let Some(text) = text {
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

    if !plugin.prefixes.is_empty() && !plugin.commands.is_empty() {
        let mut map = COMMAND_MAP.write().unwrap();
        for prefix in plugin.prefixes {
            for cmd in plugin.commands {
                let mut key = String::with_capacity(prefix.len() + cmd.len());
                key.push_str(prefix);
                key.push_str(cmd);
                map.entry(key).or_insert(plugin);
            }
        }
    }
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
