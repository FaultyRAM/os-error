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
use winapi::shared::minwindef::{DWORD, LPCVOID};
use winapi::shared::ntdef::WCHAR;
use winapi::um::winbase::{self, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
#[cfg(feature = "windows-desktop")]
use winapi::shared::winerror::FACILITY_NT_BIT;
#[cfg(feature = "windows-desktop")]
use winapi::um::libloaderapi;
#[cfg(feature = "windows-desktop")]
use winapi::um::winbase::FORMAT_MESSAGE_FROM_HMODULE;

#[inline]
/// Windows-specific `from_raw_os_error` implementation.
pub fn from_raw_os_error(code: i32) -> OsError {
    let mut msg: [u16; 2048] = [0; 2048];
    let msg_len = format_message_ex(code, &mut msg);
    let desc = if msg_len == 0 {
        let ec_s = code.to_string();
        let fm_ec_s = os_error_code::get_last_error().to_string();
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

#[cfg_attr(feature = "clippy", allow(cast_sign_loss))]
#[cfg(feature = "windows-desktop")]
#[inline]
/// `FormatMessageW` wrapper for desktop apps.
fn format_message_ex(code: i32, msg: &mut [u16]) -> u32 {
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
        let ntdll_module = libloaderapi::GetModuleHandleW(NTDLL_DLL.as_ptr());
        if ntdll_module.is_null() {
            error_code = code;
            ntdll_flag = 0;
        } else {
            error_code = code ^ FACILITY_NT_BIT;
            ntdll_flag = FORMAT_MESSAGE_FROM_HMODULE;
        }
        format_message(
            ntdll_flag | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ntdll_module as *mut _,
            error_code as DWORD,
            msg,
        )
    }
}

#[cfg_attr(feature = "clippy", allow(cast_sign_loss))]
#[cfg(not(feature = "windows-desktop"))]
#[inline]
/// `FormatMessageW` wrapper for store apps.
fn format_message_ex(code: i32, msg: &mut [u16]) -> u32 {
    format_message(
        FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
        ptr::null(),
        code as DWORD,
        msg,
    )
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
