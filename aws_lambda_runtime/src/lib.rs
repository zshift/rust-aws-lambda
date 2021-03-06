//! Runtime environment for AWS Lambda.
//!
//! This crate contains a runtime which will listen for messages from
//! the lambda environment, and call a handler function every time the
//! lambda is invoked.
//!
//! This handler function can be async, as the runtime itself is based on
//! top of `futures` and `tokio`.

#![deny(warnings)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(test)]
extern crate partial_io;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate backtrace_parser;
extern crate bytes;
extern crate failure;
#[macro_use]
extern crate futures;
extern crate gob;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate scoped_tls;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_schema;
#[macro_use]
extern crate serde_schema_derive;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_reactor;
extern crate tokio_service;
extern crate void;

mod error;
mod proto;
mod runtime;
mod server;

pub mod context;
pub mod env;

pub use context::Context;
pub use error::RuntimeError;
pub use runtime::Runtime;
