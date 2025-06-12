//! Procedural macros for Nebula workflow engine

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate proc_macro;

mod action;
mod credential;
mod node;
mod parameters;
mod resource;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derives the `Parameters` trait for parameter definitions
#[proc_macro_derive(
    Parameters,
    attributes(
        text,
        textarea,
        select,
        multi_select,
        radio,
        checkbox,
        secret,
        file,
        color,
        date,
        datetime,
        time,
        button,
        hidden,
        notice,
        group,
        mode,
        display,
        validation
    )
)]
pub fn derive_parameters(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match parameters::derive_parameters_impl(ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}


pub(crate) fn nebula_core_path() -> syn::Path {
    syn::parse_quote!(nebula_core)
}
