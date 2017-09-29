// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Windows-specific implementation details.

use OsError;
use kernel32;
use std::ptr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::{FORMAT_MESSAGE_FROM_HMODULE, FORMAT_MESSAGE_FROM_SYSTEM,
             FORMAT_MESSAGE_IGNORE_INSERTS};

#[cfg_attr(feature = "clippy", allow(cast_possible_wrap))]
#[inline]
/// Windows-specific `last_os_error_code` implementation.
pub fn last_os_error_code() -> i32 {
    unsafe { kernel32::GetLastError() as i32 }
}

#[inline]
/// Windows-specific `from_raw_os_error` implementation.
pub fn from_raw_os_error(code: i32) -> OsError {
    let mut msg: [u16; 2048] = [0; 2048];
    let msg_len = format_message(code, &mut msg);
    let desc = if msg_len == 0 {
        let ec_s = code.to_string();
        let fm_ec_s = last_os_error_code().to_string();
        concat_string!(
            "OS Error ",
            ec_s,
            " (FormatMessageW() returned error ",
            fm_ec_s,
            ")"
        )
    } else if let Some(m) = OsString::from_wide(&msg).to_str() {
        m.trim_right().to_owned()
    } else {
        let ec_s = code.to_string();
        concat_string!(
            "OS Error ",
            ec_s,
            " (FormatMessageW() returned invalid wide string)"
        )
    };
    OsError {
        code: code,
        desc: desc,
    }
}

#[cfg(feature = "windows-desktop")]
#[cfg_attr(feature = "clippy", allow(cast_possible_truncation))]
#[cfg_attr(feature = "clippy", allow(cast_sign_loss))]
#[inline]
/// `FormatMessageW` wrapper for desktop apps.
fn format_message(code: i32, msg: &mut [u16]) -> u32 {
    /// "NTDLL.DLL" in UTF-16.
    const NTDLL_DLL: &'static [u16] = &[
        'N' as _,
        'T' as _,
        'D' as _,
        'L' as _,
        'L' as _,
        '.' as _,
        'D' as _,
        'L' as _,
        'L' as _,
        0,
    ];
    let error_code;
    let ntdll_flag;
    unsafe {
        let ntdll_module = kernel32::GetModuleHandleW(NTDLL_DLL.as_ptr());
        if ntdll_module.is_null() {
            error_code = code;
            ntdll_flag = 0;
        } else {
            error_code = code ^ 0x1000_0000; // `FACILITY_NT_BIT` - not in winapi 0.2.8
            ntdll_flag = FORMAT_MESSAGE_FROM_HMODULE;
        }
        kernel32::FormatMessageW(
            ntdll_flag | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ntdll_module as *const _,
            error_code as u32,
            0,
            msg.as_mut_ptr(),
            msg.len() as u32,
            ptr::null_mut(),
        )
    }
}

#[cfg(not(feature = "windows-desktop"))]
#[cfg_attr(feature = "clippy", allow(cast_possible_truncation))]
#[cfg_attr(feature = "clippy", allow(cast_sign_loss))]
#[inline]
/// `FormatMessageW` wrapper for Windows Store apps.
fn format_message(code: i32, msg: &mut [u16]) -> u32 {
    unsafe {
        kernel32::FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null(),
            error_code,
            0,
            msg,
            msg.len() as u32,
            ptr::null(),
        )
    }
}
