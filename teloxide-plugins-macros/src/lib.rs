#![allow(non_snake_case)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn TeloxidePlugin(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let args_str = args.to_string();

    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let ctor_fn_name = syn::Ident::new(&format!("{}_ctor", fn_name_str), fn_name.span());
    let static_name = syn::Ident::new(&format!("{}_meta", fn_name_str), fn_name.span());
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let mut commands = Vec::new();
    let mut prefixes = Vec::new();
    let mut regex = None;
    let mut callback_filter = None;

    if args_str.contains("commands = [") {
        if let Some(start) = args_str.find("commands = [") {
            if let Some(end) = args_str[start..].find("]") {
                let commands_str = &args_str[start + 12..start + end];
                commands = commands_str
                    .split(',')
                    .map(|s| s.trim().trim_matches('"'))
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }
    
    if args_str.contains("prefixes = [") {
        if let Some(start) = args_str.find("prefixes = [") {
            if let Some(end) = args_str[start..].find("]") {
                let prefixes_str = &args_str[start + 12..start + end];
                prefixes = prefixes_str
                    .split(',')
                    .map(|s| s.trim().trim_matches('"'))
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }
    
    if args_str.contains("regex = [") {
        if let Some(start) = args_str.find("regex = [") {
            if let Some(end) = args_str[start..].find("]") {
                let regex_str = &args_str[start + 9..start + end];
                let regex_patterns: Vec<&str> = regex_str
                    .split(',')
                    .map(|s| s.trim().trim_matches('"'))
                    .filter(|s| !s.is_empty())
                    .collect();
                regex = regex_patterns.first().copied();
            }
        }
    }
    
    if args_str.contains("callback = [") {
        if let Some(start) = args_str.find("callback = [") {
            if let Some(end) = args_str[start..].find("]") {
                let callback_str = &args_str[start + 12..start + end];
                let callback_patterns: Vec<&str> = callback_str
                    .split(',')
                    .map(|s| s.trim().trim_matches('"'))
                    .filter(|s| !s.is_empty())
                    .collect();
                callback_filter = callback_patterns.first().copied();
            }
        }
    }

    let commands_lit: Vec<proc_macro2::TokenStream> =
        commands.iter().map(|c| quote! { #c }).collect();
    let prefixes_lit: Vec<proc_macro2::TokenStream> =
        prefixes.iter().map(|p| quote! { #p }).collect();
    let regex_lit = match regex {
        Some(r) => quote! { Some(#r) },
        None => quote! { None },
    };
    let callback_filter_lit = match callback_filter {
        Some(c) => quote! { Some(#c) },
        None => quote! { None },
    };
    
    let callback_handler = if callback_filter.is_some() {
        quote! {
            |ctx| Box::pin(async move {
                if let Some(cq) = ctx.callback_query.clone() {
                    #fn_name(ctx.bot.clone(), cq).await;
                }
            })
        }
    } else {
        quote! {
            |ctx| Box::pin(async move {
                if let Some(msg) = ctx.message.clone() {
                    #fn_name(ctx.bot.clone(), msg).await;
                }
            })
        }
    };

    let expanded = quote! {
        #vis #sig #block

        #[allow(non_upper_case_globals)]
        #[doc(hidden)]
        static #static_name: &teloxide_plugins::registry::PluginMeta = &teloxide_plugins::registry::PluginMeta {
            name: #fn_name_str,
            commands: &[#(#commands_lit),*],
            prefixes: &[#(#prefixes_lit),*],
            regex: #regex_lit,
            callback_filter: #callback_filter_lit,
            callback: #callback_handler,
        };

        #[ctor::ctor]
        fn #ctor_fn_name() {
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                handle.spawn(async move {
                    let mut reg = teloxide_plugins::registry::PluginRegistry.lock().await;
                    reg.push(#static_name);
                });
            } else {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async {
                        let mut reg = teloxide_plugins::registry::PluginRegistry.lock().await;
                        reg.push(#static_name);
                    });
            }
        }
    };

    TokenStream::from(expanded)
}
