// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Windows-specific implementation details.

use os_error_code;
use std::ptr;
use OsError;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::shared::minwindef::{DWORD, HMODULE, LPCVOID};
use winapi::shared::ntdef::WCHAR;
use winapi::um::winbase::{self, FORMAT_MESSAGE_FROM_HMODULE, FORMAT_MESSAGE_FROM_SYSTEM,
                          FORMAT_MESSAGE_IGNORE_INSERTS};

#[inline]
/// Windows-specific `from_raw_os_error` implementation.
pub fn from_raw_os_error(code: i32) -> OsError {
    OsError::with_windows_modules(code, &[])
}

#[cfg_attr(feature = "clippy", allow(cast_possible_truncation))]
#[inline]
/// `FormatMessageW` wrapper.
fn format_message(flags: DWORD, module: LPCVOID, code: DWORD, msg: &mut [WCHAR]) -> u32 {
    unsafe {
        winbase::FormatMessageW(
            flags,
            module,
            code,
            0,
            msg.as_mut_ptr(),
            msg.len() as DWORD,
            ptr::null_mut(),
        )
    }
}

impl OsError {
    #[cfg_attr(feature = "clippy", allow(cast_sign_loss))]
    #[cfg_attr(feature = "clippy", allow(indexing_slicing))]
    /// Attempts to retrieve an error description from one of the given Windows modules, before
    /// falling back to checking the system.
    ///
    /// On Windows, many error codes do not necessarily come from the system, but from e.g. system
    /// drivers and various low-level APIs. Users expecting such error codes must therefore specify
    /// where the relevant error descriptions reside.
    pub fn with_windows_modules(code: i32, modules: &[HMODULE]) -> Self {
        let mut msg: [u16; 2048] = [0; 2048];
        for module in modules {
            let msg_len = format_message(
                FORMAT_MESSAGE_FROM_HMODULE | FORMAT_MESSAGE_IGNORE_INSERTS,
                *module as *const _,
                code as DWORD,
                &mut msg,
            );
            if msg_len > 0 {
                return Self::build_os_error(code, &msg[..msg_len as usize]);
            }
        }
        let msg_len = format_message(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null(),
            code as DWORD,
            &mut msg,
        );
        if msg_len > 0 {
            Self::build_os_error(code, &msg[..msg_len as usize])
        } else {
            let ec_s = code.to_string();
            let fm_ec_s = os_error_code::get_last_error().to_string();
            let desc = concat_string!(
                "OS Error ",
                ec_s,
                " (FormatMessageW() returned error ",
                fm_ec_s,
                ")"
            );
            Self {
                code: code,
                desc: desc,
            }
        }
    }

    #[inline]
    /// Converts an error code into an `OsError`.
    fn build_os_error(code: i32, msg: &[u16]) -> Self {
        let desc = if let Some(m) = OsString::from_wide(msg).to_str() {
            m.trim_right().to_owned()
        } else {
            let ec_s = code.to_string();
            concat_string!(
                "OS Error ",
                ec_s,
                " (FormatMessageW() returned invalid wide string)"
            )
        };
        Self {
            code: code,
            desc: desc,
        }
    }
}
