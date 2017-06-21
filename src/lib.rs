#![allow(dead_code)]
#![feature(alloc)]
#![feature(allocator_api)]
#![no_std]

extern crate alloc;

pub mod protocol;
mod void;
mod base;
mod guid;
mod table;
mod systemtable;
mod bootservices;
mod runtimeservices;
mod console;
mod task;
mod event;
pub mod util;
mod allocator;


pub use base::{Handle, Handles, Event, MemoryType, Status, Time};
pub use guid::*;

pub use systemtable::*;

pub use bootservices::BootServices;

pub use runtimeservices::{ResetType, RuntimeServices};

pub use console::{Attribute, ForegroundColor, BackgroundColor, InputKey, SimpleTextOutput, SimpleTextInput, Console};

pub use event::*;

pub use task::*;

pub use void::CVoid;

