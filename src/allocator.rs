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

use alloc::allocator::*;

use base::Status;

/// Rust Allocator utilizing the EFI {allocate, free}_pool functions.
struct EfiAllocator {}

unsafe impl<'a> Alloc for &'a EfiAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        ::get_system_table()
            .boot_services()
            .allocate_pool(layout.size())
            .map_err(|e| match e {
                         Status::OutOfResources => AllocErr::Exhausted { request: layout },
                         x => AllocErr::Unsupported { details: x.str() },
                     })
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        ::get_system_table().boot_services().free_pool(ptr);
    }
}
