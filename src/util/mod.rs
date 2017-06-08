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
pub fn utf16_ptr_to_str(chars: *const u16) -> Result<&'static str, Status> { 
    let strlen = utf16_strlen(chars);

    let raw_u8_ptr: Result<*mut u8, Status> = ::get_system_table().boot_services().allocate_pool(strlen);
    if let Err(status) = raw_u8_ptr {
        return Err(status);
    }
    let raw_u8_ptr = raw_u8_ptr.unwrap();

    for i in 0..strlen as isize {
        unsafe {
            *(raw_u8_ptr.offset(i)) = *(chars.offset(i)) as u8;
        }
    }

    let u8_slice = unsafe { slice::from_raw_parts(raw_u8_ptr, strlen) };
    unsafe {
        Ok(str::from_utf8_unchecked(u8_slice))
    }
}

pub fn str_to_utf16_ptr(chars: &str) -> Result<*const u16, Status> {
    ::get_system_table()
        .boot_services()
        .allocate_pool(chars.len() + 1)
        .map(|u16_ptr| {
            for i in 0..chars.len() as isize {
                unsafe {
                    *(u16_ptr.offset(i)) = *(chars.as_ptr().offset(i));
                }
            }
            unsafe { *(u16_ptr.offset(chars.len() as isize)) = 0 };
            u16_ptr as *const u16
        })
}
