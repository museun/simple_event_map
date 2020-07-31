use async_channel::Receiver;
use futures_lite::Stream;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

pin_project_lite::pin_project! {
    /// A stream of events
    ///
    /// This can be used both as a _blocking_ `Iterator` and an _async_ `Stream`
    pub struct EventStream<T> {
        #[pin]
        pub(crate) inner: Receiver<T>,
    }
}

impl<T> std::fmt::Debug for EventStream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventStream")
            .field("element_type", &std::any::type_name::<T>())
            .finish()
    }
}

impl<T> Iterator for EventStream<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.try_recv().ok()
    }
}

impl<T> Stream for EventStream<T> {
    type Item = T;
    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(ctx)
    }
}
