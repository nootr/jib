//! # Jib
//!
//! A Jib to Javascript compiler.
//!
//! This library will contain a [lexer], [parser] and generator to compile from Jib to Javascript.
//! For more information about Jib, see [Jib's README file on Github][jib].
//!
//! [jib]: https://github.com/nootr/jib

#![doc(
    html_logo_url = "https://github.com/nootr/jib/assets/16090423/1d886605-d485-4f7a-be3f-82642f14823e"
)]
#![warn(missing_docs)]

pub mod lexer;
pub mod parser;
