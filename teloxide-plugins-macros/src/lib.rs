#![allow(non_snake_case)]

use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, Expr, ExprArray,
    ExprLit, ItemFn, Lit, LitStr, Meta, MetaNameValue, Token,
};

const COMMANDS_IDENT: &str = "commands";
const PREFIXES_IDENT: &str = "prefixes";
const REGEX_IDENT: &str = "regex";
const CALLBACK_IDENT: &str = "callback";

struct PluginArgs {
    metas: Punctuated<Meta, Token![,]>,
}

impl Parse for PluginArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(PluginArgs {
            metas: Punctuated::parse_terminated(input)?,
        })
    }
}

fn extract_strings_from_array(expr: &Expr) -> syn::Result<Vec<String>> {
    match expr {
        Expr::Array(ExprArray { elems, .. }) => {
            let mut strings = Vec::new();
            for elem in elems {
                match elem {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }) => {
                        strings.push(lit_str.value());
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            elem,
                            "expected string literal in array",
                        ));
                    }
                }
            }
            Ok(strings)
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "expected array of string literals",
        )),
    }
}

fn create_optional_string_literal(value: Option<&String>) -> proc_macro2::TokenStream {
    value
        .map(|s| {
            let lit_str = LitStr::new(s, proc_macro2::Span::call_site());
            quote! { Some(#lit_str) }
        })
        .unwrap_or_else(|| quote! { None })
}

fn parse_plugin_args(
    args: TokenStream,
) -> syn::Result<(Vec<String>, Vec<String>, Option<String>, Option<String>)> {
    let mut commands = Vec::new();
    let mut prefixes = Vec::new();
    let mut regex = None;
    let mut callback_filter = None;

    let plugin_args: PluginArgs = syn::parse(args)?;

    for meta in plugin_args.metas {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            if let Some(ident) = path.get_ident() {
                match ident.to_string().as_str() {
                    COMMANDS_IDENT => {
                        commands = extract_strings_from_array(&value)?;
                    }
                    PREFIXES_IDENT => {
                        prefixes = extract_strings_from_array(&value)?;
                    }
                    REGEX_IDENT => {
                        let patterns = extract_strings_from_array(&value)?;
                        if !patterns.is_empty() {
                            if patterns.len() == 1 {
                                regex = Some(patterns[0].clone());
                            } else {
                                let combined_pattern = patterns.join("|");
                                regex = Some(combined_pattern);
                            }
                        }
                    }
                    CALLBACK_IDENT => {
                        let patterns = extract_strings_from_array(&value)?;
                        if !patterns.is_empty() {
                            if patterns.len() == 1 {
                                callback_filter = Some(patterns[0].clone());
                            } else {
                                let combined_pattern = patterns.join("|");
                                callback_filter = Some(combined_pattern);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok((commands, prefixes, regex, callback_filter))
}

fn determine_handler_type(
    commands: &[String],
    prefixes: &[String],
    regex: &Option<String>,
    callback_filter: &Option<String>,
) -> syn::Result<bool> {
    let has_message_triggers = !commands.is_empty() || !prefixes.is_empty() || regex.is_some();
    let has_callback_triggers = callback_filter.is_some();

    match (has_message_triggers, has_callback_triggers) {
        (true, true) => {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "plugin cannot handle both message triggers (commands/prefixes/regex) and callback triggers simultaneously"
            ))
        }
        (true, false) => Ok(false),
        (false, true) => Ok(true),
        (false, false) => {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "plugin must specify at least one trigger: commands, prefixes, regex, or callback"
            ))
        }
    }
}

fn create_callback_handler(fn_name: &syn::Ident, is_callback: bool) -> proc_macro2::TokenStream {
    if is_callback {
        quote! {
            |ctx| Box::pin(async move {
                if let Some(cq) = ctx.callback_query {
                    #fn_name(ctx.bot, cq).await;
                }
            })
        }
    } else {
        quote! {
            |ctx| Box::pin(async move {
                if let Some(msg) = ctx.message {
                    #fn_name(ctx.bot, msg).await;
                }
            })
        }
    }
}

#[proc_macro_attribute]
pub fn TeloxidePlugin(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let ctor_fn_name = syn::Ident::new(&format!("{}_ctor", fn_name_str), fn_name.span());
    let static_name = syn::Ident::new(&format!("{}_meta", fn_name_str), fn_name.span());
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let (commands, prefixes, regex, callback_filter) = match parse_plugin_args(args) {
        Ok(result) => result,
        Err(err) => return err.to_compile_error().into(),
    };

    let is_callback_handler =
        match determine_handler_type(&commands, &prefixes, &regex, &callback_filter) {
            Ok(is_callback) => is_callback,
            Err(err) => return err.to_compile_error().into(),
        };

    let commands_lit = commands
        .iter()
        .map(|c| LitStr::new(c, proc_macro2::Span::call_site()));
    let prefixes_lit = prefixes
        .iter()
        .map(|p| LitStr::new(p, proc_macro2::Span::call_site()));
    let regex_lit = create_optional_string_literal(regex.as_ref());
    let callback_filter_lit = create_optional_string_literal(callback_filter.as_ref());

    let callback_handler = create_callback_handler(fn_name, is_callback_handler);

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
            teloxide_plugins::registry::register_plugin(#static_name);
        }
    };

    TokenStream::from(expanded)
}
