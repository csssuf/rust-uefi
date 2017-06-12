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

use base::Status;
use protocol::{DevicePathProtocol, DevicePathUtilitiesProtocol};
use void::CVoid;

pub fn create_file_device_node(filename: &str) -> Result<&DevicePathProtocol, Status> {
    let node_size_bytes = 4 + (filename.len() + 1) * 2;

    ::get_system_table()
        .boot_services()
        .locate_protocol::<DevicePathUtilitiesProtocol>(0 as *const CVoid)
        .and_then(|utilities| {
            utilities.create_device_node(4, 4, node_size_bytes as u16)
                .map(|node_ptr| {
                    let file_name_ptr: *mut u16 = unsafe { (node_ptr as *const u8).offset(4) as *mut u16 };

                    for i in 0..filename.len() as isize{
                        unsafe {
                            *file_name_ptr.offset(i) = *filename.as_ptr().offset(i) as u16;
                        }
                    }
                    unsafe { *file_name_ptr.offset(filename.len() as isize) = 0 };

                    unsafe { &*node_ptr }
                })
        })
}
