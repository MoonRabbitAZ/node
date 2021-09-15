

//! Approval DB accessors and writers for on-disk persisted approval storage
//! data.
//!
//! We persist data to disk although it is not intended to be used across runs of the
//! program. This is because under medium to long periods of finality stalling, for whatever
//! reason that may be, the amount of data we'd need to keep would be potentially too large
//! for memory.
//!
//! With tens or hundreds of parachains, hundreds of validators, and parablocks
//! in every relay chain block, there can be a humongous amount of information to reference
//! at any given time.
//!
//! As such, we provide a function from this module to clear the database on start-up.
//! In the future, we may use a temporary DB which doesn't need to be wiped, but for the
//! time being we share the same DB with the rest of Substrate.

pub mod v1;
