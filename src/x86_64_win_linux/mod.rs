#[cfg(target_os = "linux")]
mod linux_allocator;

mod hf_alloc;
pub use hf_alloc::*;

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
fn hf_func_wrapper<const N: usize, F: FnOnce([u8; N], *mut u8) -> *mut u8>(f: F) {
    let mut buffer = [0u8; N];
    // TODO: Stop using pop here, we should read this the
    //       same way IrOp::StackPop pops from the stack
    unsafe {
        for i in 0..N {
            // asm!("pop {}", out(reg_byte) buffer[i]);
            asm!("mov al, byte ptr[rsp]");
            asm!("mov {}, al", out(reg_byte) buffer[i]);
            asm!("lea rsp, [rsp+1]");
        }
    }
    let mut r8_value: *mut u8;
    unsafe {
        asm!("mov {}, r8", out(reg) r8_value);
    }
    r8_value = f(buffer, r8_value);
    unsafe {
        asm!("mov r8, {}", in(reg) r8_value);
    }
}

const MEM_SIZE_DEFAULT: usize = 50 * 1024 * 1024;

static mut MEM_PTR: *mut u8 = core::ptr::null_mut();
static mut MEM_SIZE: usize = MEM_SIZE_DEFAULT;

#[no_mangle]
pub extern "C" fn hf_start() {
    unsafe {
        MEM_PTR = alloc::alloc::alloc(alloc::alloc::Layout::from_size_align_unchecked(
            MEM_SIZE_DEFAULT,
            1,
        ));

        asm!("mov r8, {}", in(reg) MEM_PTR as u64);
    }
}

#[no_mangle]
pub extern "C" fn hf_exit() {
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
