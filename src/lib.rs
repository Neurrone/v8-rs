// #![feature(uniform_paths)]
//! A high-level wrapper around the [V8 Javascript engine][1].
//!
//!
//! [1]: https://developers.google.com/v8/

#[macro_use()]
extern crate lazy_static;

pub mod context;
// pub mod error;
pub mod handle;
pub mod isolate;
//pub mod script;
// pub mod template;
// pub mod value;

pub use self::context::Context;
pub use self::isolate::Isolate;
//pub use script::Script;
//pub use value::Value;
