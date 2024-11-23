#[cfg(target_os = "linux")]
mod linux_allocator;

mod hf_alloc;
pub use hf_alloc::*;

mod hf_print;
pub use hf_print::*;

use alloc::vec::Vec;
use core::arch::asm;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(target_os = "linux")]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "int3",
            "mov rax, 60",  // syscall number for exit
            "mov rdi, 134", // exit code for SIGABRT
            "syscall",
            options(noreturn)
        )
    }
}

#[cfg(target_os = "windows")]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "int3",
            "mov rax, 0x3C",  // syscall number for exit
            "mov rdi, 0x1F4", // exit code for SIGABRT
            "syscall",
            options(noreturn)
        )
    }
}

/// A beautiful, magical function that saves the r8 register, pops
/// N values from the stack into a buffer, calls the provided function
/// with the buffer, and restores the r8 register.
fn hf_func_wrapper<const N: usize, F: FnOnce([u8; N], *mut u8) -> *mut u8>(
    r8: *mut *mut u8,
    stack_ptr: *mut *mut u8,
    f: F,
) {
    let mut buffer = [0u8; N];
    unsafe {
        let start_ptr = STACK.as_ptr() as u64;
        let stack_index = (*stack_ptr as u64) - start_ptr;
        buffer.copy_from_slice(&STACK[(stack_index - N as u64) as usize..stack_index as usize]);
        *stack_ptr.wrapping_sub(N);
        *r8 = f(buffer, *r8);
    }
}

const MEM_SIZE_DEFAULT: usize = 50 * 1024 * 1024;

static mut MEM_PTR: *mut u8 = core::ptr::null_mut();
static mut MEM_SIZE: usize = MEM_SIZE_DEFAULT;

static mut STACK: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub extern "C" fn hf_start(r8: *mut *mut u8, stack_ptr: *mut *mut u8) {
    unsafe {
        *stack_ptr = STACK.as_mut_ptr();
    }

    unsafe {
        MEM_PTR = alloc::alloc::alloc(alloc::alloc::Layout::from_size_align_unchecked(
            MEM_SIZE_DEFAULT,
            1,
        ));

        *r8 = MEM_PTR;
    }
}

/// We do return stack_ptr but it really doesn't matter, hf_exit_impl will
/// never return anyway.
#[no_mangle]
pub extern "C" fn hf_exit(_r8: *mut *mut u8, _stack_ptr: *mut *mut u8) {
    unsafe {
        alloc::alloc::dealloc(
            MEM_PTR,
            alloc::alloc::Layout::from_size_align_unchecked(MEM_SIZE, 1),
        );

        hf_exit_impl();
    }
}

#[cfg(target_os = "linux")]
unsafe fn hf_exit_impl() {
    asm!(
        "mov rax, 60", // syscall number for exit
        "mov rdi, 0",
        "syscall",
        options(noreturn)
    )
}

#[cfg(target_os = "windows")]
unsafe fn hf_exit_impl() {
    asm!(
        "mov rax, 0x3C", // syscall number for exit
        "mov rdi, 0",
        "syscall",
        options(noreturn)
    )
}
