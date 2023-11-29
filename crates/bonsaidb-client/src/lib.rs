#![doc = include_str!(".crate-docs.md")]
#![cfg_attr(not(feature = "included-from-omnibus"), doc = include_str!("../client-feature-flags.md"))]
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms,
)]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
)]

pub use url;

mod builder;
mod client;
mod error;

#[cfg(not(target_arch = "wasm32"))]
pub use fabruic;

#[cfg(not(target_arch = "wasm32"))]
pub use self::builder::Blocking;
pub use self::builder::{Async, Builder};
pub use self::client::{ApiCallback, AsyncClient, AsyncRemoteDatabase, AsyncRemoteSubscriber};
#[cfg(not(target_arch = "wasm32"))]
pub use self::client::{BlockingClient, BlockingRemoteDatabase, BlockingRemoteSubscriber};
pub use self::error::{ApiError, Error};
