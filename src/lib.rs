//! UEFI library for Rust
//!
//! This library provides a Rust-friendly interface to the functions provided in a UEFI application
//! environment. For complete documentation on UEFI programming, refer to the UEFI
//! specification[1].
//!
//! [1]: http://www.uefi.org/specifications
//!
//! ## Usage
//!
//! Building Rust applications for UEFI is a bit tricky. Rust doesn't support outputting binaries
//! in the EFI application format, so the best way to build Rust EFI applications is to use the
//! Rust tooling to build an object file which is linked and formatted by another tool. The easiest
//! way to do this is to use a Makefile which builds the application's object file via cargo, links
//! it as if it were a gnuefi application, and copies it to the correct format. For example:
//!
//! ### Example Makefile
//!
//! ```text
//! LIB_DIR ?= /usr/lib64
//! EFI_DIR ?= $(LIB_DIR)/gnuefi
//!
//! all: TYPE := release
//! all: CARGO_FLAG := --release
//! all: build/bootx64-release.efi
//! .PHONY: all
//!
//! debug: TYPE := debug
//! debug: CARGO_FLAG :=
//! debug: build/bootx64-debug.efi
//! .PHONY: debug
//!
//! build/bootx64-%.efi: build/APP_NAME-%.so
//!     objcopy -j .text -j .sdata -j .data -j .dynamic -j .dynsym -j .rel -j .rela -j .reloc \
//!         --target=efi-app-x86_64 $< $@
//!
//! build/APP_NAME-%.so: target/%/deps/APP_NAME.o
//!     @mkdir -p build
//!     ld target/$(TYPE)/deps/*.o $(EFI_DIR)/crt0-efi-x86_64.o -nostdlib -znocombreloc \
//!         -T $(EFI_DIR)/elf_x86_64_efi.lds -shared -Bsymbolic -lefi -L $(LIB_DIR) -pie \
//!         -e efi_entry -o $@
//!
//! target/%/deps/APP_NAME.o: src/APP_NAME.rs ... Cargo.toml
//!     cargo build $(CARGO_FLAG)
//!
//! clean:
//!     -rm build/*
//!     -cargo clean
//! .PHONY: clean
//! ```
//!
//! `LIB_DIR` should point to the location of libgnuefi.a on your system. If the `gnuefi` directory
//! is not a subdirectory of `LIB_DIR`, the `EFI_DIR` variable should be specified as well.
//!
//! ### Application Structure
//!
//! Something you'll notice in the above Makefile is the use of the `-e efi_entry` flag during
//! linking. This specifies that `efi_entry` should be the entry point to the application. To that
//! end, your application should provide that entrypoint somewhere. Its signature looks like this:
//!
//! ```rust,ignore
//! #[no_mangle]
//! pub extern "win64" fn efi_entry(image_handle: uefi::Handle,
//!                                 system_table: *const uefi::SystemTable)
//!                                 -> isize
//! ```
//!
//! At this point, your application should initialize this library using the two arguments it
//! receives:
//!
//! ```rust,ignore
//! uefi::set_system_table(system_table);
//! uefi::protocol::set_current_image(image_handle);
//! ```
//!
//! See [set_system_table] and [set_current_image].
//!
//! [set_system_table]: fn.set_system_table.html
//! [set_current_image]: protocol/fn.set_current_image.html
//!

#![allow(dead_code)]
#![no_std]

#[macro_use] extern crate bitflags;

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


pub use base::{Handle, Handles, Event, MemoryType, Status, Time};
pub use guid::*;

pub use systemtable::*;

pub use bootservices::*;

pub use runtimeservices::{ResetType, RuntimeServices};

pub use console::{Attribute, ForegroundColor, BackgroundColor, InputKey, SimpleTextOutput, SimpleTextInput, Console};

pub use event::*;

pub use task::*;

pub use void::CVoid;

