use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_channel::oneshot::Receiver;
use futures_lite::FutureExt;
use objc2::rc::Retained as Id;
use objc2::ClassType;
use objc2_foundation::NSObject;

pub struct AppKitFuture<T: Default> {
    recv: Receiver<T>,
    retained: Vec<Id<NSObject>>,
}

unsafe impl<T: Default> Send for AppKitFuture<T> {}
unsafe impl<T: Default> Sync for AppKitFuture<T> {}

impl<T: Default> Future for AppKitFuture<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.recv.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(x) => Poll::Ready(x.unwrap_or_default()),
        }
    }
}

impl<T: Default> AppKitFuture<T> {
    pub fn from_oneshot(recv: Receiver<T>) -> Self {
        Self {
            recv,
            retained: vec![],
        }
    }

    pub fn retain<U>(mut self, object: Id<U>) -> Self
    where
        U: ClassType<Super = NSObject> + 'static,
    {
        self.retained.push(object.into_super());
        self
    }
}
