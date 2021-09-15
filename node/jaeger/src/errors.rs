

//! moonrabbit Jaeger error definitions.

/// A description of an error during jaeger initialization.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum JaegerError {
	#[error("Already launched the collector thread")]
	AlreadyLaunched,

	#[error("Missing jaeger configuration")]
	MissingConfiguration,
}
