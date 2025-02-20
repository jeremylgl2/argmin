// Copyright 2018-2023 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # spectator
//!
//! Spectator is a dedicated GUI program which displays the progress of an optimization run.
//! The data is fed to spectator by the `argmin-observer-spectator`.
//!
//! ## Installation
//!
//! ```bash
//! cargo install spectator
//! ```
//!
//! ## Library
//!
//! This crate can also be used as a library and exposes the [`Message`] type used to encode data
//! sent to specator and [`DEFAULT_PORT`] which defines the default port used by spectator.
//!
//! # License
//!
//! Licensed under either of
//!
//!   * Apache License, Version 2.0,
//!     ([LICENSE-APACHE](https://github.com/argmin-rs/argmin/blob/main/LICENSE-APACHE) or
//!     <http://www.apache.org/licenses/LICENSE-2.0>)
//!   * MIT License ([LICENSE-MIT](https://github.com/argmin-rs/argmin/blob/main/LICENSE-MIT) or
//!     <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
//! in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
//! without any additional terms or conditions.

#![warn(missing_docs)]

mod message;

pub use message::Message;

/// Default port used by spectator
pub const DEFAULT_PORT: u16 = 5498;
