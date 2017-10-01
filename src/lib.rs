// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Utilities for handling and describing platform-specific errors.

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy_internal))]
#![cfg_attr(feature = "clippy", deny(clippy_pedantic))]
#![cfg_attr(feature = "clippy", deny(clippy_restrictions))]
#![forbid(warnings)]
#![forbid(anonymous_parameters)]
#![forbid(box_pointers)]
#![forbid(fat_ptr_transmutes)]
#![forbid(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(trivial_casts)]
#![forbid(trivial_numeric_casts)]
#![forbid(unused_extern_crates)]
#![forbid(unused_import_braces)]
#![deny(unused_qualifications)]
#![forbid(unused_results)]
#![forbid(variant_size_differences)]

#[macro_use(concat_string)]
extern crate concat_string;
#[cfg(unix)]
extern crate libc;
extern crate os_error_code;
#[cfg(windows)]
extern crate winapi;

#[cfg(unix)]
#[path = "unix.rs"]
mod sys;
#[cfg(windows)]
#[path = "windows.rs"]
mod sys;

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::result;

/// A type alias for `Result<T, OsError>`, provided for convenience.
pub type Result<T> = result::Result<T, OsError>;

#[derive(Clone, Debug)]
/// An error type providing human-readable descriptions of platform-specific errors.
pub struct OsError {
    /// The raw error code.
    code: i32,
    /// A human-readable description of the error.
    desc: String,
}

impl OsError {
    /// Creates an `OsError` from the most recent platform-specific error that occurred.
    pub fn from_last_os_error() -> Self {
        sys::from_raw_os_error(os_error_code::get_last_error())
    }

    /// Creates an `OsError` from a raw platform-specific error code.
    pub fn from_raw_os_error(code: i32) -> Self {
        sys::from_raw_os_error(code)
    }

    #[inline]
    /// Returns the raw error code associated with a platform-specific error.
    pub fn error_code(&self) -> i32 {
        self.code
    }

    #[inline]
    /// Returns a reference to a string slice describing a platform-specific error.
    pub fn as_str(&self) -> &str {
        &self.desc
    }
}

impl AsRef<str> for OsError {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for OsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.desc.fmt(f)
    }
}

impl Error for OsError {
    fn description(&self) -> &str {
        &self.desc
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
