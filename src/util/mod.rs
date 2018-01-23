// Copyright 2017 CoreOS, Inc.
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod device_path;
pub use self::device_path::*;

use core::slice;
use core::str;

use base::Status;

/// Take a null-terminated UTF-16 string (such as one returned by EFI functions) and determine its
/// length.
pub fn utf16_strlen(c: *const u16) -> usize {
    let mut len: usize = 0;
    
    unsafe {
        while *(c.offset(len as isize)) != 0 {
            len += 1;
        }
    }

    len
}

/// Convert a raw pointer to a UTF-16 string to a rust &str.
/// Note: This function expects to receive a fully ASCII-compatible string. If it does not, it will
/// fail.
pub fn utf16_ptr_to_str(chars: *const u16) -> Result<&'static str, Status> { 
    let strlen = utf16_strlen(chars);

    let raw_u8_ptr: Result<*mut u8, Status> = ::get_system_table().boot_services().allocate_pool(strlen);
    if let Err(status) = raw_u8_ptr {
        return Err(status);
    }
    let raw_u8_ptr = raw_u8_ptr.unwrap();

    for i in 0..strlen as isize {
        unsafe {
            // If the character is not ASCII, fail.
            if *(chars.offset(i)) >= 128 {
                ::get_system_table().boot_services().free_pool(raw_u8_ptr);
                return Err(Status::InvalidParameter);
            }

            *(raw_u8_ptr.offset(i)) = *(chars.offset(i)) as u8;
        }
    }

    let u8_slice = unsafe { slice::from_raw_parts(raw_u8_ptr, strlen) };
    unsafe {
        Ok(str::from_utf8_unchecked(u8_slice))
    }
}

/// Convert a rust &str to a pointer to a UTF-16 string.
/// Note: This function expects to receive a fully ASCII-compatible string. If it does not, it will
/// fail.
pub fn str_to_utf16_ptr(chars: &str) -> Result<*const u16, Status> {
    ::get_system_table()
        .boot_services()
        .allocate_pool(chars.len() + 1)
        .and_then(|u16_ptr| {
            for (i, c) in chars.chars().enumerate() {
                if c.len_utf8() > 1 {
                    ::get_system_table().boot_services().free_pool(u16_ptr);
                    return Err(Status::Unsupported);
                }

                unsafe {
                    *((u16_ptr as *mut u16).offset(i as isize)) = c as u16;
                }
            }
            unsafe { *((u16_ptr as *mut u16).offset(chars.len() as isize)) = 0 };

            Ok(u16_ptr as *const u16)
        })
}
