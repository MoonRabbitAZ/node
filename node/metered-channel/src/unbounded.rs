

//! Metered variant of unbounded mpsc channels to be able to extract metrics.

use futures::{channel::mpsc, task::Poll, task::Context, sink::SinkExt, stream::Stream};

use std::result;
use std::pin::Pin;

use super::Meter;


/// Create a wrapped `mpsc::channel` pair of `MeteredSender` and `MeteredReceiver`.
pub fn unbounded<T>() -> (UnboundedMeteredSender<T>, UnboundedMeteredReceiver<T>) {
	let (tx, rx) = mpsc::unbounded();
	let shared_meter = Meter::default();
	let tx = UnboundedMeteredSender { meter: shared_meter.clone(), inner: tx };
	let rx = UnboundedMeteredReceiver { meter: shared_meter, inner: rx };
	(tx, rx)
}

/// A receiver tracking the messages consumed by itself.
#[derive(Debug)]
pub struct UnboundedMeteredReceiver<T> {
	// count currently contained messages
	meter: Meter,
	inner: mpsc::UnboundedReceiver<T>,
}

impl<T> std::ops::Deref for UnboundedMeteredReceiver<T> {
	type Target = mpsc::UnboundedReceiver<T>;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> std::ops::DerefMut for UnboundedMeteredReceiver<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T> Stream for UnboundedMeteredReceiver<T> {
	type Item = T;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		match mpsc::UnboundedReceiver::poll_next(Pin::new(&mut self.inner), cx) {
			Poll::Ready(x) => {
				self.meter.note_received();
				Poll::Ready(x)
			}
			other => other,
		}
	}

	/// Don't rely on the unreliable size hint.
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}
}

impl<T> UnboundedMeteredReceiver<T> {
	/// Get an updated accessor object for all metrics collected.
	pub fn meter(&self) -> &Meter {
		&self.meter
	}

	/// Attempt to receive the next item.
	pub fn try_next(&mut self) -> Result<Option<T>, mpsc::TryRecvError> {
		match self.inner.try_next()? {
			Some(x) => {
				self.meter.note_received();
				Ok(Some(x))
			}
			None => Ok(None),
		}
	}
}

impl<T> futures::stream::FusedStream for UnboundedMeteredReceiver<T> {
	fn is_terminated(&self) -> bool {
		self.inner.is_terminated()
	}
}


/// The sender component, tracking the number of items
/// sent across it.
#[derive(Debug)]
pub struct UnboundedMeteredSender<T> {
	meter: Meter,
	inner: mpsc::UnboundedSender<T>,
}

impl<T> Clone for UnboundedMeteredSender<T> {
	fn clone(&self) -> Self {
		Self { meter: self.meter.clone(), inner: self.inner.clone() }
	}
}

impl<T> std::ops::Deref for UnboundedMeteredSender<T> {
	type Target = mpsc::UnboundedSender<T>;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> std::ops::DerefMut for UnboundedMeteredSender<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T> UnboundedMeteredSender<T> {
	/// Get an updated accessor object for all metrics collected.
	pub fn meter(&self) -> &Meter {
		&self.meter
	}

	/// Send message, wait until capacity is available.
	pub async fn send(&mut self, item: T) -> result::Result<(), mpsc::SendError>
	where
		Self: Unpin,
	{
		self.meter.note_sent();
		let fut = self.inner.send(item);
		futures::pin_mut!(fut);
		fut.await.map_err(|e| {
			self.meter.retract_sent();
			e
		})
	}


	/// Attempt to send message or fail immediately.
	pub fn unbounded_send(&self, msg: T) -> result::Result<(), mpsc::TrySendError<T>> {
		self.meter.note_sent();
		self.inner.unbounded_send(msg).map_err(|e| {
			self.meter.retract_sent();
			e
		})
	}
}

impl<T> futures::sink::Sink<T> for UnboundedMeteredSender<T> {
	type Error = <futures::channel::mpsc::UnboundedSender<T> as futures::sink::Sink<T>>::Error;

	fn start_send(mut self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
		Pin::new(&mut self.inner).start_send(item)
	}

	fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		Pin::new(&mut self.inner).poll_ready(cx)
	}

	fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		match Pin::new(&mut self.inner).poll_ready(cx) {
			val @ Poll::Ready(_)=> {
				val
			}
			other => other,
		}
	}

	fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		match Pin::new(&mut self.inner).poll_ready(cx) {
			val @ Poll::Ready(_)=> {
				self.meter.note_sent();
				val
			}
			other => other,
		}
	}
}
