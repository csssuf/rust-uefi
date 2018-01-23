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
use void::CVoid;

#[repr(C)]
#[derive(Debug)]
pub struct BlockIOMedia {
    media_id: u32,
    removable: u8,
    present: u8,
    logical_partition: u8,
    read_only: u8,
    write_caching: u8,
    block_size: u32,
    pub io_align: u32,
    pub last_block: u64,
    lowest_aligned_lba: u64,
    logical_blocks_per_physical_block: u32,
    optimal_transfer_length_granularity: u32,
}

pub static EFI_BLOCK_IO_PROTOCOL_GUID: Guid = Guid(
    0x964E_5B21,
    0x6459,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

/// Bindings to the EFI Block I/O protocol. This protocol provides synchronous access to block
/// devices, and allows block-by-block access.
#[repr(C)]
pub struct BlockIOProtocol {
    revision: u64,
    pub media: *const BlockIOMedia,
    reset: unsafe extern "win64" fn(this: *const BlockIOProtocol, extended_verification: u8)
        -> Status,
    read_blocks: unsafe extern "win64" fn(
        this: *const BlockIOProtocol,
        media_id: u32,
        lba: u64,
        buffer_size: usize,
        buffer: *mut CVoid,
    ) -> Status,
    write_blocks: unsafe extern "win64" fn(
        this: *const BlockIOProtocol,
        media_id: u32,
        lba: u64,
        buffer_size: usize,
        buffer: *const CVoid,
    ) -> Status,
    flush_blocks: unsafe extern "win64" fn(this: *const BlockIOProtocol) -> Status,
}

impl Protocol for BlockIOProtocol {
    fn guid() -> &'static Guid {
        &EFI_BLOCK_IO_PROTOCOL_GUID
    }
}

impl BlockIOProtocol {
    /// Indicates whether or not the device is removable.
    pub fn is_removable(&self) -> bool {
        unsafe { (*self.media).removable == 1 }
    }

    /// Indicates whether or not the device is present.
    pub fn is_present(&self) -> bool {
        unsafe { (*self.media).present == 1 }
    }

    /// Indicates whether or not the device is a logical partition.
    pub fn is_logical_partition(&self) -> bool {
        unsafe { (*self.media).logical_partition == 1 }
    }

    /// Indicates whether or not the device is read only.
    pub fn is_read_only(&self) -> bool {
        unsafe { (*self.media).read_only == 1 }
    }

    /// Indicates whether or not the device performs write caching.
    pub fn write_caching(&self) -> bool {
        unsafe { (*self.media).write_caching == 1 }
    }

    /// Indicates whether or not the device has alignment requirements for buffers.
    pub fn must_align(&self) -> bool {
        unsafe { (*self.media).io_align > 1 }
    }

    /// Calculates the required number of pages to read `blocks` blocks from this block device.
    pub fn required_pages_block(&self, blocks: usize) -> usize {
        let block_size = unsafe { (*self.media).block_size } as usize;
        let bytes = block_size * blocks;

        self.required_pages(bytes)
    }

    /// Calculates the required number of pages to read `read_size` bytes from this block device.
    pub fn required_pages(&self, read_size: usize) -> usize {
        let block_size = unsafe { (*self.media).block_size } as usize;
        let mut actual_read_size = read_size;

        if read_size % block_size != 0 {
            actual_read_size = block_size * ((read_size / block_size) + 1);
        }

        let mut num_pages = actual_read_size / 4096;
        if actual_read_size % 4096 != 0 {
            num_pages += 1;
        }

        num_pages
    }

    /// Free some data read by this protocol.
    pub fn free_read(&self, buffer: &[u8]) {
        ::get_system_table().boot_services().free_pages(
            buffer.as_ptr(),
            self.required_pages(buffer.len()),
        );
    }

    /// Reset the device.
    pub fn reset(&self, extended_verification: bool) -> Result<(), Status> {
        match unsafe { (self.reset)(self, extended_verification as u8) } {
            Status::Success => Ok(()),
            e => Err(e),
        }
    }

    /// Read `num_bytes` bytes from the disk starting at block `start`. The returned slice includes
    /// memory allocated with `allocate_pages`, and it is the caller's responsibility to free it
    /// with `free_read`.
    pub fn read_bytes(&self, start: u64, num_bytes: usize) -> Result<&mut [u8], Status> {
        let bs = ::get_system_table().boot_services();
        let mut read_size = num_bytes;
        let buffer: Result<*mut u8, Status>;

        // Reads can only be performed in multiples of the block size, so round up to the nearest
        // block.
        let block_size = unsafe { (*self.media).block_size } as usize;
        if num_bytes % block_size != 0 {
            read_size = block_size * ((num_bytes / block_size) + 1);
        }

        // The read buffer must be aligned to the value of `media.io_align`. UEFI doesn't provide
        // any sort of memalign, so in order to be safe, use `allocate_pages` to obtain a 4K-aligned
        // address instead of `allocate_pool`. This isn't an ideal solution, but it does work in
        // lieu of implementing memalign and keeping track of the original allocation.
        buffer = bs.allocate_pages(self.required_pages(read_size)).map(|buf| buf as *mut u8);

        buffer.and_then(|buffer| unsafe {
            let out_slice = slice::from_raw_parts_mut(buffer, num_bytes);
            match (self.read_blocks)(
                self,
                (*self.media).media_id,
                start,
                num_bytes,
                buffer as *mut CVoid,
            ) {
                Status::Success => Ok(out_slice),
                e => {
                    self.free_read(out_slice);
                    Err(e)
                }
            }
        })
    }

    /// Read `num_blocks` blocks from the disk starting at block `start`. The returned slice
    /// includes memory allocated with `allocate_pages`, and it is the caller's responsibility to
    /// free it.
    pub fn read_blocks(&self, start: u64, num_blocks: usize) -> Result<&mut [u8], Status> {
        let block_size = unsafe { (*self.media).block_size };
        let read_size_bytes = num_blocks * block_size as usize;
        self.read_bytes(start, read_size_bytes)
    }

    /// Write `buffer` to the disk starting at block `start`. `buffer.len()` must be a multiple of
    /// the disks's block size, or else this call will fail.
    pub fn write_bytes(&self, start: u64, buffer: &[u8]) -> Result<(), Status> {
        match unsafe {
            (self.write_blocks)(
                self,
                (*self.media).media_id,
                start,
                buffer.len(),
                buffer.as_ptr() as *const CVoid,
            )
        } {
            Status::Success => Ok(()),
            e => Err(e),
        }
    }

    /// Flush any pending writes to this disk.
    pub fn flush_blocks(&self) -> Result<(), Status> {
        match unsafe { (self.flush_blocks)(self) } {
            Status::Success => Ok(()),
            e => Err(e),
        }
    }
}
