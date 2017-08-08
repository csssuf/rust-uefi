use core::{default, fmt, ptr, slice};

use void::CVoid;

/// Type for EFI_HANDLE.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Handle(pub(crate) *mut CVoid);

impl default::Default for Handle {
    fn default() -> Handle { Handle(ptr::null_mut()) }
}

#[derive(Debug)]
pub struct Handles(*const Handle, usize);

impl Handles {
    pub fn new(p: *const Handle, len: usize) -> Handles {
        return Handles(p, len);
    }
}

#[cfg(target_os = "efi")]
impl ::core::ops::Drop for Handles {
	fn drop(&mut self) {
        let bs = systemtable::get_system_table().boot_services();
        bs.free_pool(self.0);
    }
}

impl<'a> ::core::iter::IntoIterator for &'a Handles {
    type Item = &'a Handle;
    type IntoIter = HandlesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        HandlesIterator {
            handles: self,
            offset: 0,
        }
    }
}

pub struct HandlesIterator<'a> {
    handles: &'a Handles,
    offset: usize,
}

impl<'a> ::core::iter::Iterator for HandlesIterator<'a> {
    type Item = &'a Handle;

    fn next(&mut self) -> Option<Self::Item> {
        let sl = unsafe { slice::from_raw_parts(self.handles.0, self.handles.1) };
        let item = sl.get(self.offset);
        self.offset += 1;
        return item;
    }
}

impl<'a> ::core::iter::ExactSizeIterator for HandlesIterator<'a> {
    fn len(&self) -> usize {
        self.handles.1
    }
}

/// Type for EFI_EVENT.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Event(pub *mut CVoid);

#[cfg(target_pointer_width = "32")]
const ERR_FLAG: u32 = 1 << 31;

#[cfg(target_pointer_width = "64")]
const ERR_FLAG: u64 = 1 << 63;

/// Type for EFI_STATUS
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Status {
    Success = 0,
    LoadError = 1 | ERR_FLAG,
    InvalidParameter = 2 | ERR_FLAG,
    Unsupported = 3 | ERR_FLAG,
    BadBufferSize = 4 | ERR_FLAG,
    BufferTooSmall = 5 | ERR_FLAG,
    NotReady = 6 | ERR_FLAG,
    DeviceError = 7 | ERR_FLAG,
    WriteProtected = 8 | ERR_FLAG,
    OutOfResources = 9 | ERR_FLAG,
    VolumeCorrupted = 10 | ERR_FLAG,
    VolumeFull = 11 | ERR_FLAG,
    NoMedia = 12 | ERR_FLAG,
    MediaChanged = 13 | ERR_FLAG,
    NotFound = 14 | ERR_FLAG,
    AccessDenied = 15 | ERR_FLAG,
    NoResponse = 16 | ERR_FLAG,
    NoMapping = 17 | ERR_FLAG,
    Timeout = 18 | ERR_FLAG,
    NotStarted = 19 | ERR_FLAG,
    AlreadyStarted = 20 | ERR_FLAG,
    Aborted = 21 | ERR_FLAG,
    IcmpError = 22 | ERR_FLAG,
    TftpError = 23 | ERR_FLAG,
    ProtocolError = 24 | ERR_FLAG,
    IncompatibleVersion = 25 | ERR_FLAG,
    SecurityViolation = 26 | ERR_FLAG,
    CrcError = 27 | ERR_FLAG,
    EndOfMedia = 28 | ERR_FLAG,
    EndOfFile = 31 | ERR_FLAG,
}

impl Status {
    pub fn str(&self) -> &'static str {
        match *self {
            Status::Success => "success",
            Status::LoadError => "load error",
            Status::InvalidParameter => "invalid parameter",
            Status::Unsupported => "unsupported",
            Status::BadBufferSize => "bad buffer size",
            Status::BufferTooSmall => "buffer too small",
            Status::NotReady => "not ready",
            Status::DeviceError => "device error",
            Status::WriteProtected => "write protected",
            Status::OutOfResources => "out of resources",
            Status::VolumeCorrupted => "volume corrupted",
            Status::VolumeFull => "volume full",
            Status::NoMedia => "no media",
            Status::MediaChanged => "media changed",
            Status::NotFound => "not found",
            Status::AccessDenied => "access denied",
            Status::NoResponse => "no response",
            Status::NoMapping => "no mapping",
            Status::Timeout => "timeout",
            Status::NotStarted => "not started",
            Status::AlreadyStarted => "already started",
            Status::Aborted => "aborted",
            Status::IcmpError => "ICMP error",
            Status::TftpError => "TFTP error",
            Status::ProtocolError => "protocol error",
            Status::IncompatibleVersion => "incompatible version",
            Status::SecurityViolation => "security violation",
            Status::CrcError => "CRC error",
            Status::EndOfMedia => "end of media",
            Status::EndOfFile => "end of file",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

#[test]
fn status_str() {
    assert_eq!(Status::Success.str(), "success");
}

/// Type for EFI_MEMORY_TYPE
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
#[repr(C)]
pub enum MemoryType {
    Reserved = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServicesData = 6,
    Conventional = 7,
    Unusable = 8,
    AcpiReclaimed = 9,
    AcpiNvs = 10,
    MemoryMappedIo = 11,
    MemoryMappedIoPortSpace = 12,
    PalCode = 13,
}

/// Type for EFI_ALLOCATE_TYPE
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
#[repr(C)]
pub enum AllocateType {
    AnyPages = 0,
    MaxAddress = 1,
    Address = 2,
}

/// UEFI Time structure.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Time {
    /// Year [1900 - 9999]
    pub year: u16,

    /// Month [1 - 12]
    pub month: u8,

    /// Day [1 - 31]
    pub day: u8,

    /// Hour [0 - 23]
    pub hour: u8,

    /// Minute [0 - 59]
    pub minute: u8,

    /// Second [0 - 59]
    pub second: u8,

    __pad1: u8,

    /// Nanosecond [0 - 999,999,999]
    pub nanosecond: u32,

    /// Timezone [-1440 - 1440] or 2047 for "unspecified timezone"
    pub timezone: i16,

    daylight: u8,
    __pad2: u8,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

#[repr(C)]
pub struct TimeCapabilities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: bool,
}

