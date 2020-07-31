//! # simple_event_stream
//!
//! This allows you to register types that'll be broadcast to receivers
//!
//! ## An example
//! ```rust
//! # use simple_event_map::{EventMap, EventStream};
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
//! // this returns an `EventStream`
//! // which can be used as a blocking Iterator or as an async Stream
//! let mut m1 = map.register::<Message>();
//! let mut m2 = map.register::<Message>();
//!
//! let msg = Message{ data: String::from("hello world") };
//! // send the message, will return a bool if any messages were sent
//! assert_eq!(map.send(msg.clone()), true);
//! // we should have 2 still active
//! assert_eq!(map.active::<Message>(), 2);
//!
//! assert_eq!(m1.next().unwrap(), msg);
//! assert_eq!(m2.next().unwrap(), msg);
//!
//! // drop a subscription (will be cleaned up in the eventmap on next send)
//! drop(m1);
//!
//! let msg = Message{ data: String::from("testing") };
//! assert_eq!(map.send(msg.clone()), true);
//! // we only have 1 active now
//! assert_eq!(map.active::<Message>(), 1);
//! ```

mod channel;
pub use channel::Sender;

mod event_stream;
pub use event_stream::EventStream;

mod event_map;

pub use event_map::{EventMap, Senders};
