

//! Error types for the subsystem requests.

/// A description of an error causing the runtime API request to be unservable.
#[derive(Debug, Clone)]
pub struct RuntimeApiError(String);

impl From<String> for RuntimeApiError {
	fn from(s: String) -> Self {
		RuntimeApiError(s)
	}
}

impl core::fmt::Display for RuntimeApiError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		write!(f, "{}", self.0)
	}
}

impl std::error::Error for RuntimeApiError {}

/// A description of an error causing the chain API request to be unservable.
#[derive(Debug, Clone)]
pub struct ChainApiError {
	msg: String,
}

impl From<&str> for ChainApiError {
	fn from(s: &str) -> Self {
		s.to_owned().into()
	}
}

impl From<String> for ChainApiError {
	fn from(msg: String) -> Self {
		Self { msg }
	}
}

impl core::fmt::Display for ChainApiError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		write!(f, "{}", self.msg)
	}
}

impl std::error::Error for ChainApiError {}

/// An error that may happen during Availability Recovery process.
#[derive(PartialEq, Debug, Clone)]
pub enum RecoveryError {
	/// A chunk is recovered but is invalid.
	Invalid,

	/// A requested chunk is unavailable.
	Unavailable,
}

impl std::fmt::Display for RecoveryError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		write!(f, "{}", self)
	}
}

impl std::error::Error for RecoveryError {}
