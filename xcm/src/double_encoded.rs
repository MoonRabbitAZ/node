

use alloc::vec::Vec;
use moonrabbit_scale_codec::{Encode, Decode};

#[derive(Encode, Decode)]
#[codec(encode_bound())]
#[codec(decode_bound())]
pub struct DoubleEncoded<T> {
	encoded: Vec<u8>,
	#[codec(skip)]
	decoded: Option<T>,
}

impl<T> Clone for DoubleEncoded<T> {
	fn clone(&self) -> Self { Self { encoded: self.encoded.clone(), decoded: None } }
}
impl<T> Eq for DoubleEncoded<T> {
}
impl<T> PartialEq for DoubleEncoded<T> {
	fn eq(&self, other: &Self) -> bool { self.encoded.eq(&other.encoded) }
}
impl<T> core::fmt::Debug for DoubleEncoded<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.encoded.fmt(f) }
}

impl<T> From<Vec<u8>> for DoubleEncoded<T> {
	fn from(encoded: Vec<u8>) -> Self {
		Self { encoded, decoded: None }
	}
}

impl<T> DoubleEncoded<T> {
	pub fn into<S>(self) -> DoubleEncoded<S> { DoubleEncoded::from(self) }
	pub fn from<S>(e: DoubleEncoded<S>) -> Self {
		Self {
			encoded: e.encoded,
			decoded: None,
		}
	}
	pub fn as_ref(&self) -> Option<&T> {
		self.decoded.as_ref()
	}
}

impl<T: Decode> DoubleEncoded<T> {
	pub fn ensure_decoded(&mut self) -> Result<&T, ()> {
		if self.decoded.is_none() {
			self.decoded = T::decode(&mut &self.encoded[..]).ok();
		}
		self.decoded.as_ref().ok_or(())
	}
	pub fn take_decoded(&mut self) -> Result<T, ()> {
		self.decoded.take().or_else(|| T::decode(&mut &self.encoded[..]).ok()).ok_or(())
	}
	pub fn try_into(mut self) -> Result<T, ()> {
		self.ensure_decoded()?;
		self.decoded.ok_or(())
	}
}
