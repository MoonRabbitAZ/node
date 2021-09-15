

//! moonrabbit CLI library.

#![warn(missing_docs)]

#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "cli")]
mod command;

pub use service::{
	self,
	ProvideRuntimeApi, CoreApi, IdentifyVariant,
	Block, RuntimeApiCollection, TFullClient
};

#[cfg(feature = "cli")]
pub use cli::*;

#[cfg(feature = "cli")]
pub use command::*;

#[cfg(feature = "cli")]
pub use sc_cli::{Error, Result};
