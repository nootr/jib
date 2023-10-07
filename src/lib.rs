//! # Jib
//!
//! A Jib to Javascript compiler.
//!
//! This library will contain a [lexer], parser and generator to compile from Jib to Javascript.
//! For more information about Jib, see [Jib's README file on Github][jib].
//!
//! [jib]: https://github.com/nootr/jib

#![warn(missing_docs)]

pub mod args;
pub mod lexer;

pub use crate::args::get_args;
