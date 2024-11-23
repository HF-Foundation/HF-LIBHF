#![no_std]
#![feature(allocator_api)]

#[macro_use]
extern crate alloc;

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod x86_64_win_linux;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use x86_64_win_linux::*;
