//! Message stream implementation for iFlow SDK
//!
//! This module provides the stream implementation for receiving messages from iFlow.

use crate::message::types::Message;
use futures::{FutureExt, pin_mut, Stream};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::{Mutex, mpsc};

/// Stream of messages from iFlow
///
/// This stream provides asynchronous access to messages received from iFlow.
/// It implements the `futures::Stream` trait for easy integration with async code.
pub struct MessageStream {
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<Message>>>,
}

impl MessageStream {
    /// Create a new MessageStream
    ///
    /// # Arguments
    /// * `receiver` - The receiver for messages
    pub fn new(receiver: Arc<Mutex<mpsc::UnboundedReceiver<Message>>>) -> Self {
        Self { receiver }
    }
}

impl Stream for MessageStream {
    type Item = Message;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut receiver = match self.receiver.try_lock() {
            Ok(guard) => guard,
            Err(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        // Use asynchronous receiving
        match receiver.try_recv() {
            Ok(msg) => Poll::Ready(Some(msg)),
            Err(mpsc::error::TryRecvError::Empty) => {
                // Register a waker to be notified when new messages arrive
                let recv_future = receiver.recv();
                pin_mut!(recv_future);
                match recv_future.poll_unpin(cx) {
                    Poll::Ready(msg) => Poll::Ready(msg),
                    Poll::Pending => Poll::Pending,
                }
            }
            Err(mpsc::error::TryRecvError::Disconnected) => Poll::Ready(None),
        }
    }
}