use core::ptr;
use core::mem;

use void::{NotYetDef, CVoid};
use base::{Event, Handle, Handles, MemoryType, Status};
use event::{EventType, EventNotify, TimerDelay};
use task::TPL;
use protocol::{DevicePathProtocol, Protocol, get_current_image};
use guid;
use table;

#[repr(C)]
pub enum LocateSearchType {
    AllHandles = 0,
    ByRegisterNotify = 1,
    ByProtocol = 2,
}

/// See http://wiki.phoenix.com/wiki/index.php/EFI_BOOT_SERVICES
#[repr(C)]
pub struct BootServices {
    header: table::TableHeader,
    raise_tpl: *const NotYetDef,
    restore_tpl: *const NotYetDef,
    allocate_pages: *const NotYetDef,
    free_pages: *const NotYetDef,
    get_memory_map: *const NotYetDef,
    allocate_pool: unsafe extern "win64" fn(pool_type: MemoryType, size: usize, out: *mut *mut u8) -> Status,
    free_pool: unsafe extern "win64" fn(*mut CVoid),
    create_event: unsafe extern "win64" fn(event_type: EventType, notify_tpl: TPL, notify_function: Option<EventNotify>, notify_context: *const CVoid, event: *mut Event) -> Status,
    set_timer: unsafe extern "win64" fn(event: Event, delay_type: TimerDelay, delay: u64) -> Status,
    // typedef EFI_STATUS (EFIAPI *EFI_WAIT_FOR_EVENT) (IN UINTN NumberOfEvents, IN EFI_EVENT *Event, OUT UINTN *Index);
    wait_for_event: unsafe extern "win64" fn(usize, *const Event, *mut usize) -> Status,
    signal_event: *const NotYetDef,
    close_event: *const NotYetDef,
    check_event: *const NotYetDef,
    install_protocol_interface: *const NotYetDef,
    reinstall_protocol_interface: *const NotYetDef,
    uninstall_protocol_interface: *const NotYetDef,
    handle_protocol: unsafe extern "win64" fn(Handle, &guid::Guid, &mut *mut CVoid) -> Status,
    __reserved: *const NotYetDef,
    register_protocol_notify: *const NotYetDef,
    locate_handle: *const NotYetDef,
    locate_device_path: *const NotYetDef,
    install_configuration_table: *const NotYetDef,
    load_image: unsafe extern "win64" fn(boot_policy: u8, parent_image_handle: Handle, device_path: *const DevicePathProtocol, source_buffer: *const CVoid, source_size: usize, image_handle: *mut Handle) -> Status,
    start_image: unsafe extern "win64" fn(image_handle: Handle, exit_data_size: *mut usize, exit_data: *mut *const u16) -> Status,
    exit: *const NotYetDef,
    unload_image: *const NotYetDef,
    exit_boot_services: *const NotYetDef,
    get_next_monotonic_count: *const NotYetDef,
    stall: unsafe extern "win64" fn(usize) -> Status,
    set_watchdog_timer: unsafe extern "win64" fn(timeout: usize, code: u64, data_size: usize, data: *const u16) -> Status,
    connect_controller: *const NotYetDef,
    disconnect_controller: *const NotYetDef,
    open_protocol: *const NotYetDef,
    close_protocol: unsafe extern "win64" fn(handle: Handle, protocol: &guid::Guid, agent_handle: Handle, controller_handle: Handle) -> Status,
    open_protocol_information: *const NotYetDef,
    protocols_per_handle: *const NotYetDef,
    locate_handle_buffer: unsafe extern "win64" fn(search_type: LocateSearchType, protocol: &guid::Guid, search_key: *const CVoid, nhandles: *mut usize, handles: *mut *mut CVoid) -> Status,
    locate_protocol: unsafe extern "win64" fn(protocol: &guid::Guid, registration: *const CVoid, interface: &mut *mut CVoid) -> Status,
    install_multiple_protocol_interfaces: *const NotYetDef,
    uninstall_multiple_protocol_interfaces: *const NotYetDef,
    calculate_crc32: *const NotYetDef,
    copy_mem: unsafe extern "win64" fn(*mut CVoid, *mut CVoid, usize),
    set_mem: unsafe extern "win64" fn(*mut CVoid, usize, u8),
    create_event_ex: *const NotYetDef,
}

impl BootServices {
    /// Allocate `size` bytes of memory using type `T`.
    pub fn allocate_pool<T>(&self, size: usize) -> Result<*mut T, Status> {
        let mut ptr: *mut u8 = 0 as *mut u8;

        let result = unsafe { (self.allocate_pool)(get_current_image().image_data_type, size, &mut ptr) };
        if result != Status::Success {
            return Err(result);
        }
        Ok(ptr as *mut T)
    }

    pub fn free_pool<T>(&self, p: *const T) {
        unsafe {
            (self.free_pool)(p as *mut CVoid);
        }
    }

    pub fn create_event(&self, event_type: EventType, notify_tpl: TPL, notify_func: Option<EventNotify>, notify_context: *const CVoid) -> Result<Event, Status> {
        let mut event: Event = Event(0 as *mut CVoid);

        let result = unsafe { (self.create_event)(event_type, notify_tpl, notify_func, notify_context, &mut event) };
        if result != Status::Success {
            return Err(result);
        }

        Ok(event)
    }

    pub fn set_timer(&self, event: Event, delay_type: TimerDelay, delay: u64) -> Status {
        unsafe {
            (self.set_timer)(event, delay_type, delay)
        }
    }

    pub fn wait_for_event(&self, events: &[Event]) -> Result<usize, Status> {
        // XXX: asserts sizeof *Cvoid == sizeof Event
        if false {
            use core::mem;
            unsafe { mem::transmute::<*const CVoid, Event>(0x1 as *const CVoid); }
        }

        let mut index : usize = 0;

        let result = unsafe { (self.wait_for_event)(events.len(), events.as_ptr(), &mut index) };
        if result != Status::Success {
            return Err(result);
        }

        Ok(index)
    }

    pub fn handle_protocol<T: Protocol>(&self, handle: Handle) -> Result<&'static T, Status> {
        let mut ptr : *mut CVoid = 0 as *mut CVoid;
        let guid = T::guid();


        unsafe {
            let status = (self.handle_protocol)(handle, guid, &mut ptr);
            if status != Status::Success {
                return Err(status);
            }
        }

        let r = unsafe { mem::transmute::<*mut CVoid, &'static T>(ptr) };
        Ok(r)
    }

    // TODO: for the love of types, fix me
    pub fn close_protocol<T: Protocol>(&self, handle: Handle, agent_handle: Handle, controller_handle: Handle) -> Status {
        let guid = T::guid();

        unsafe {
            (self.close_protocol)(handle, guid, agent_handle, controller_handle)
        }
    }

    /// Retrives a slice of handles by protocol GUID.
    pub fn locate_handle_by_protocol<T: Protocol>(&self) -> Result<Handles, Status> {
        let mut nhandles : usize = 0;
        let mut handles : *mut CVoid = ptr::null_mut();
        let guid = T::guid();

        let res = unsafe { (self.locate_handle_buffer)(LocateSearchType::ByProtocol, guid, ptr::null(), &mut nhandles as *mut usize, &mut handles) };

        if res != Status::Success {
            return Err(res);
        }

        return Ok(Handles::new(handles as *mut Handle, nhandles));
    }

    /// Load an image by device path and return its handle.
    pub fn load_image(&self, boot_policy: bool, parent_image_handle: Handle, device_path: *const DevicePathProtocol) -> Result<Handle, Status> {
        self.load_image_buffer(boot_policy, parent_image_handle, device_path, 0 as *const CVoid, 0)
    }

    /// Load an image already loaded into memory at source_buffer and return its handle.
    pub fn load_image_buffer(&self, boot_policy: bool, parent_image_handle: Handle, device_path: *const DevicePathProtocol, source_buffer: *const CVoid, source_size: usize) -> Result<Handle, Status> {
        let mut handle: Handle = Default::default();

        let result = unsafe { (self.load_image)(boot_policy as u8, parent_image_handle, device_path, source_buffer, source_size, &mut handle) };
        if result != Status::Success {
            return Err(result);
        }

        Ok(handle)
    }

    /// Start a loaded image, and return its ExitData.
    pub fn start_image_with_exitdata(&self, image_handle: Handle) -> Result<(*const u16, usize), Status> {
        let mut exit_data_ptr: *const u16 = 0 as *const u16;
        let mut exit_data_size: usize = 0;

        let result = unsafe { (self.start_image)(image_handle, &mut exit_data_size, &mut exit_data_ptr) };
        if result != Status::Success {
            return Err(result);
        }

        Ok((exit_data_ptr, exit_data_size))
    }

    /// Start a loaded image, but ignore its ExitData.
    pub fn start_image(&self, image_handle: Handle) -> Result<(), Status> {
        let result = unsafe { (self.start_image)(image_handle, 0 as *mut usize, 0 as *mut *const u16) };
        if result != Status::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sleep for a number of microseconds.
    pub fn stall(&self, microseconds: usize) {
        unsafe {
            (self.stall)(microseconds);
        }
    }

    /// Set or disable the watchdog timer.
    pub fn set_watchdog_timer(&self, seconds: usize, code: u64) -> Status {
        unsafe {
            (self.set_watchdog_timer)(seconds, code, 0, ptr::null())
        }
    }

    /// Find the first device handle that supports a given protocol, and return a pointer to the
    /// protocol interface from that handle.
    pub fn locate_protocol<T: Protocol>(&self, registration: *const CVoid) -> Result<&'static T, Status> {
        let mut ptr: *mut CVoid = 0 as *mut CVoid;
        let guid = T::guid();

        unsafe {
            let status = (self.locate_protocol)(guid, registration, &mut ptr);
            if status != Status::Success {
                return Err(status);
            }

            let r = mem::transmute::<*mut CVoid, &'static T>(ptr);
            Ok(r)
        }
    }

    /// Copy memory, similar to memcpy.
    pub fn copy_mem(&self, dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        unsafe {
            (self.copy_mem)(dest as *mut CVoid, src as *mut CVoid, n);
        }

        return dest;
    }

    /// Set memory, similar to memset.
    pub fn set_mem(&self, s: *mut u8, c: u8, n: usize) -> *mut u8 {
        unsafe {
            (self.set_mem)(s as *mut CVoid, n, c);
        }

        return s;
    }
}

