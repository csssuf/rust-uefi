use base::{Handle, MemoryType, Status};
use guid::Guid;
use void::NotYetDef;

mod device_path;

pub use self::device_path::*;

pub trait Protocol {
    fn guid() -> &'static Guid;
}

/// GUID for UEFI protocol for loaded images
pub static EFI_LOADED_IMAGE_PROTOCOL_GUID: Guid = Guid(0x5B1B31A1, 0x9562, 0x11d2, [0x8E,0x3F,0x00,0xA0,0xC9,0x69,0x72,0x3B]);

static mut THIS_LOADED_IMAGE: *const LoadedImageProtocol = 0 as *const LoadedImageProtocol;

#[derive(Debug)]
#[repr(C)]
pub struct LoadedImageProtocol {
    revision: u32,
    parent_handle: Handle,
    system_table: *const NotYetDef,
    pub device_handle: Handle,
    pub file_path: *const DevicePathProtocol,
    __reserved: *const NotYetDef,
    load_options_size: u32,
    load_options: *const NotYetDef,
    pub image_base: usize,
    pub image_size: u64,
    image_code_type: MemoryType,
    pub image_data_type: MemoryType,

    //unload: unsafe extern "win64" fn(handle: ::base::Handle),
    unload: *const NotYetDef,
}

impl Protocol for LoadedImageProtocol {
    fn guid() -> &'static Guid {
        return &EFI_LOADED_IMAGE_PROTOCOL_GUID;
    }
}

pub fn set_current_image(handle: Handle) -> Result<&'static LoadedImageProtocol, Status> {
    let st = ::get_system_table();

    let loaded_image_proto: Result<&'static LoadedImageProtocol, Status> = st.boot_services().handle_protocol(handle);
    if let Ok(image) = loaded_image_proto {
        unsafe {
            THIS_LOADED_IMAGE = image;
        }
    }

    loaded_image_proto
}

pub fn get_current_image() -> &'static LoadedImageProtocol {
    unsafe {
        &*THIS_LOADED_IMAGE
    }
}

