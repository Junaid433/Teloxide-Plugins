#![allow(non_upper_case_globals)]

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::sync::Arc;
use regex::Regex;
use std::future::Future;
use std::pin::Pin;
use std::collections::HashMap;
use crate::context::PluginContext;

pub struct PluginMeta {
    pub name: &'static str,
    pub commands: &'static [&'static str],
    pub prefixes: &'static [&'static str],
    pub regex: Option<&'static str>,
    pub callback_filter: Option<&'static str>,
    pub callback: fn(PluginContext) -> Pin<Box<dyn Future<Output=()> + Send>>,
}

pub static PluginRegistry: Lazy<Arc<Mutex<Vec<&'static PluginMeta>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

static REGEX_CACHE: Lazy<Arc<Mutex<HashMap<&'static str, Regex>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub async fn dispatch(ctx: PluginContext) -> Result<(), teloxide::RequestError> {
    let text = ctx.message.as_ref().and_then(|m| m.text());
    let cb_data = ctx
        .callback_query
        .as_ref()
        .and_then(|c| c.data.as_deref());

    let plugins: Vec<&'static PluginMeta> = {
        let registry = PluginRegistry.lock().await;
        registry.clone()
    };
    
    for plugin in plugins {
        if let Some(text) = text {
            let mut command_strings = Vec::new();
            for prefix in plugin.prefixes {
                for cmd in plugin.commands {
                    command_strings.push(format!("{}{}", prefix, cmd));
                }
            }
            
            for cmd_str in &command_strings {
                if text == *cmd_str {
                    (plugin.callback)(ctx.clone()).await;
                    return Ok(());
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

async fn get_or_compile_regex(pattern: &'static str) -> Regex {
    {
        let cache = REGEX_CACHE.lock().await;
        if let Some(regex) = cache.get(pattern) {
            return regex.clone();
        }
    }
    
    let regex = Regex::new(pattern).unwrap_or_else(|_| {
        Regex::new("(?!)").unwrap()
    });
    
    {
        let mut cache = REGEX_CACHE.lock().await;
        cache.insert(pattern, regex.clone());
    }
    
    regex
}
