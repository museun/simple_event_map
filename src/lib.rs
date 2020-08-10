//! # simple_event_stream
//!
//! This allows you to register types that'll be broadcast to receivers
//!
//! ## An example
//! ```rust
//! # use simple_event_map::{EventMap, EventStream};
//! # use futures_lite::StreamExt as _;
//! # futures_lite::future::block_on(async move {
//! #[derive(Clone, Debug, PartialEq)]
//! struct Message { data: String }
//!
//! let mut map = EventMap::new();
//! // nothing is registered by default
//! assert_eq!(map.is_empty::<i32>(), true);
//! assert_eq!(map.is_empty::<String>(), true);
//! assert_eq!(map.is_empty::<Message>(), true);
//!
//! // register two subscriptions for the message
//! // you can get a blocking iterator
//! let mut m1 = map.register_iter::<Message>();
//! // or you can get an async stream
//! let mut m2 = map.register_stream::<Message>();
//!
//! let msg = Message{ data: String::from("hello world") };
//! // send the message, will return a bool if any messages were sent
//! assert_eq!(map.send(msg.clone()), true);
//! // we should have 2 still active
//! assert_eq!(map.active::<Message>(), 2);
//!
//! assert_eq!(m1.next().unwrap(), msg);
//! // m2 is a stream, so we have to await it (and use StreamExt::next)
//! assert_eq!(m2.next().await.unwrap(), msg);
//!
//! // drop a subscription (will be cleaned up in the eventmap on next send)
//! drop(m1);
//!
//! let msg = Message{ data: String::from("testing") };
//! assert_eq!(map.send(msg.clone()), true);
//! // we only have 1 active now
//! assert_eq!(map.active::<Message>(), 1);
//! # });
//! ```

mod channel;
pub use channel::Sender;

mod stream;
pub use stream::EventStream;

mod iter;
pub use iter::EventIter;

mod event_map;

pub use event_map::{EventMap, Senders};
