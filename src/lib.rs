// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/otprs/refs/heads/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/opensass/otprs/refs/heads/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "dio")]
pub mod dioxus;

#[cfg(feature = "lep")]
pub mod leptos;

pub mod common;

pub use common::{
    InputMode, OtpValidationError, Variant, generate_totp, hotp, pattern, validate_hotp,
    validate_totp,
};

#[cfg(any(feature = "lep", feature = "dio", feature = "yew"))]
pub use common::unix_now_secs;

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
