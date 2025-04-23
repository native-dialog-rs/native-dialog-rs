use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_channel::oneshot::Receiver;
use futures_lite::FutureExt;
use objc2::rc::Retained as Id;
use objc2::runtime::AnyObject;

pub struct DispatchResponse<T> {
    receiver: Receiver<T>,
    retained: Vec<Id<AnyObject>>,
}

unsafe impl<T: Send> Send for DispatchResponse<T> {}
unsafe impl<T: Sync> Sync for DispatchResponse<T> {}

impl<T: Default> Future for DispatchResponse<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.receiver.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(x) => Poll::Ready(x.unwrap_or_default()),
        }
    }
}

impl<T: Default> DispatchResponse<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Self {
            receiver,
            retained: vec![],
        }
    }

    pub fn retain(mut self, object: impl Into<Id<AnyObject>>) -> Self {
        self.retained.push(object.into());
        self
    }
}
