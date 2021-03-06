// Convenience importing of `debug!`/`info!` macros for entire crate.
#[macro_use]
extern crate log;

pub use self::error::{Error,
                      Result};

pub mod binlink;
pub mod config;
pub mod crypto;
pub mod env;
pub mod error;
pub mod fs;
pub mod os;
pub mod package;
pub mod service;
pub mod url;
pub mod util;

use std::fmt;

use serde_derive::{Deserialize,
                   Serialize};

pub use crate::os::{filesystem,
                    users};

pub const AUTH_TOKEN_ENVVAR: &str = "HAB_AUTH_TOKEN";

// A Builder channel
#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ChannelIdent(String);

impl env::Config for ChannelIdent {
    const ENVVAR: &'static str = "HAB_BLDR_CHANNEL";
}

impl ChannelIdent {
    pub fn as_str(&self) -> &str { self.0.as_str() }

    pub fn stable() -> Self { Self::from("stable") }

    pub fn unstable() -> Self { Self::from("unstable") }
}

impl From<&str> for ChannelIdent {
    fn from(s: &str) -> Self { ChannelIdent(s.to_string()) }
}

impl From<String> for ChannelIdent {
    fn from(s: String) -> Self { ChannelIdent(s) }
}

impl std::str::FromStr for ChannelIdent {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> { Ok(Self::from(s)) }
}

impl fmt::Display for ChannelIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl Default for ChannelIdent {
    fn default() -> Self { Self::stable() }
}
