// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Unix-specific implementation details.

use libc;
use OsError;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

pub fn from_raw_os_error(code: i32) -> OsError {
    let mut msg = [0; 128];
    unsafe {
        if libc::strerror_r(code, msg.as_mut_ptr() as *mut _, msg.len()) == 0 {
            let desc = if let Some(s) = OsStr::from_bytes(&msg).to_str() {
                s.to_owned()
            } else {
                let ec_s = code.to_string();
                concat_string!("OS Error ", ec_s, " (strerror_r() returned invalid UTF-8)")
            };
            OsError {
                code: code,
                desc: desc,
            }
        } else {
            let ec_s = code.to_string();
            concat_string!("OS Error ", ec_s, " (strerror_r() failed)")
        }
    }
}
