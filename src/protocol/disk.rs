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
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied
// See the License for the specific language governing permissions and
// limitations under the License.

use core::slice;

use base::Status;
use guid::Guid;
use protocol::Protocol;
use void::{CVoid, NotYetDef};

pub static EFI_DISK_IO_PROTOCOL_GUID: Guid = Guid(
    0xCE34_5171,
    0xBA0B,
    0x11D2,
    [0x8E, 0x4F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

/// Bindings to the EFI Disk I/O protocol. This protocol is a synchronous abstraction on top of the
/// Block I/O protocol, and allows accessing arbitrary offsets/lengths instead of the block-based
/// accesses the Block I/O protocol provides.
#[repr(C)]
pub struct DiskIOProtocol {
    revision: u64,
    read_disk: unsafe extern "win64" fn(this: *const DiskIOProtocol,
                                        media_id: u32,
                                        offset: u64,
                                        buffer_size: usize,
                                        buffer: *mut CVoid)
                                        -> Status,
    write_disk: *const NotYetDef,
}

impl Protocol for DiskIOProtocol {
    fn guid() -> &'static Guid {
        &EFI_DISK_IO_PROTOCOL_GUID
    }
}

impl DiskIOProtocol {
    /// Read data from the disk at the given offset and size. `media_id` should be derived from the
    /// Block I/O protocol (see specifically the `BlockIOMedia` struct). The returned slice
    /// includes memory allocated with `allocate_pool`, and it is the caller's responsibility to
    /// free it.
    pub fn read_disk(&self, media_id: u32, offset: u64, size: usize) -> Result<&[u8], Status> {
        ::get_system_table()
            .boot_services()
            .allocate_pool::<u8>(size)
            .and_then(|buffer| unsafe {
                match (self.read_disk)(self, media_id, offset, size, buffer as *mut CVoid) {
                    Status::Success => Ok(slice::from_raw_parts(buffer, size)),
                    e => Err(e),
                }
            })
    }
}
