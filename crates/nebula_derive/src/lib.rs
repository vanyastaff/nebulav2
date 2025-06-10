//! Procedural macros for Nebula workflow engine

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate proc_macro;

use utils::*;

mod parameters;
mod action;
mod node;
mod credential;
mod resource;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derives the `Parameters` trait for parameter definitions
#[proc_macro_derive(Parameters, attributes(
    text, textarea, select, multi_select, radio, checkbox, secret, file,
    color, date, datetime, time, button, hidden, notice, group, mode,
    display, validation
))]
pub fn derive_parameters(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match parameters::derive_parameters_impl(ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derives the `Action` trait for workflow actions
#[proc_macro_derive(Action, attributes(action))]
pub fn derive_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match action::derive_action_impl(ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derives the `Node` trait for workflow nodes
#[proc_macro_derive(Node, attributes(node))]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match node::derive_node_impl(ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derives the `Credential` trait for authentication
#[proc_macro_derive(Credential, attributes(credential))]
pub fn derive_credential(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match credential::derive_credential_impl(ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derives the `Resource` trait for shared resources
#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match resource::derive_resource_impl(ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

pub(crate) fn nebula_core_path() -> syn::Path {
    syn::parse_quote!(nebula_core)
}