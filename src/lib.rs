#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]

mod client;
pub mod endpoints;
mod error;
pub mod models;

pub use client::{TelnyxClient, TelnyxClientBuilder};
pub use error::TelnyxError;
